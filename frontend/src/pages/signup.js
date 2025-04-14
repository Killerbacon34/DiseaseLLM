import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Fingerprint2 from 'fingerprintjs2';
import { useRouter } from 'next/navigation';

const Signup = () => {
    const router = useRouter();

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const [deviceId, setDeviceId] = useState('');

    useEffect(() => {
        if (window.requestIdleCallback) {
            requestIdleCallback(() => {
                Fingerprint2.get((components) => {
                    const values = components.map(component => component.value);
                    const murmur = Fingerprint2.x64hash128(values.join(''), 31);
                    setDeviceId(murmur);
                    console.log('Device ID:', murmur);
                })  ;
            });
        } else {
            setTimeout(() => {
                Fingerprint2.get((components) => {
                    const values = components.map(component => component.value);
                    const murmur = Fingerprint2.x64hash128(values.join(''), 31);
                    setDeviceId(murmur);
                });
            }, 500);
        }
    }, []);

    const handleSignup = async (e) => {
        e.preventDefault();
        try {
            const response = await axios.post('http://localhost:4545/api/signup', {
                username: username,
                role: 0, 
                pass: password,
                origdevid: deviceId,
            }, {
                headers: {
                    'Content-Type': 'application/json',
                },
            });
            const token = response.data;
            localStorage.setItem('token', token);
            setError('');
            alert('Signup successful!');
            router.push('/release?new=true');
        } catch (err) {
            setError('Signup failed. Please try again.');
            console.log(err);
        }
    };

    // return (
    //     <div className="d-flex align-items-center justify-content-center vh-100">
    //         <div>
    //         <h2 className="text-center mb-3">Signup</h2>
    //         <form onSubmit={handleSignup}>
    //             <div className="mb-3">
    //                 <label className="form-label">Username:</label>
    //                 <input
    //                     type="text"
    //                     value={username}
    //                     onChange={(e) => setUsername(e.target.value)}
    //                     required
    //                     className="form-control"
    //                 />
    //             </div>
    //             <div className="mb-3">
    //                 <label className="form-label">Password:</label>
    //                 <input
    //                     type="password"
    //                     value={password}
    //                     onChange={(e) => setPassword(e.target.value)}
    //                     required
    //                     className="form-control"
    //                 />
    //             </div>
    //             {error && <p className="text-danger text-center" style={{ color: 'red' }}>{error}</p>}
    //             <button type="submit" className="btn btn-primary w-100">Signup</button>
    //         </form>
    //         </div>
    //     </div>
    // );
    
    return (
        <div className="bg-light min-vh-100 d-flex align-items-center">
            <div className="container py-5">
                <div className="row justify-content-center">
                    <div className="col-md-8 col-lg-6 col-xl-5">
                        <div className="card shadow-sm border-0">
                            <div className="card-body p-4 p-md-5">
                                <div className="text-center mb-4">
                                    <h2 className="fw-bold mb-2">Create Account</h2>
                                    <p className="text-muted">Join our medical diagnostics platform</p>
                                </div>
    
                                {error && (
                                    <div className="alert alert-danger text-center">
                                        {error}
                                    </div>
                                )}
    
                                <form onSubmit={handleSignup}>
                                    <div className="mb-3">
                                        <label className="form-label fw-medium">Username</label>
                                        <input
                                            type="text"
                                            value={username}
                                            onChange={(e) => setUsername(e.target.value)}
                                            required
                                            className="form-control py-2"
                                            placeholder="Enter your username"
                                        />
                                    </div>
    
                                    <div className="mb-4">
                                        <label className="form-label fw-medium">Password</label>
                                        <input
                                            type="password"
                                            value={password}
                                            onChange={(e) => setPassword(e.target.value)}
                                            required
                                            className="form-control py-2"
                                            placeholder="Create a password"
                                        />
                                        <div className="form-text">Use 8 or more characters with a mix of letters, numbers & symbols</div>
                                    </div>
    
                                    <button 
                                        type="submit" 
                                        className="btn btn-primary w-100 py-2 mb-3 fw-medium"
                                    >
                                        Sign Up
                                    </button>
    
                                    <div className="text-center mt-3">
                                        <p className="text-muted mb-0">
                                            Already have an account?{' '}
                                            <a 
                                                href="" 
                                                className="text-primary text-decoration-none fw-medium"
                                                onClick={(e) => {
                                                    e.preventDefault();
                                                    router.push('/login');
                                                }}
                                            >
                                                Log in
                                            </a>
                                        </p>
                                    </div>
                                </form>
                            </div>
                        </div>
    
                        <div className="text-center mt-4">
                            <small className="text-muted">
                                By signing up, you agree to our Terms of Service and Privacy Policy
                            </small>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};


export default Signup;