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
            router.push('/release');
        } catch (err) {
            setError('Signup failed. Please try again.');
            console.log(err);
        }
    };

    return (
        <div className="d-flex align-items-center justify-content-center vh-100">
            <div>
            <h2 className="text-center mb-3">Signup</h2>
            <form onSubmit={handleSignup}>
                <div className="mb-3">
                    <label className="form-label">Username:</label>
                    <input
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        required
                        className="form-control"
                    />
                </div>
                <div className="mb-3">
                    <label className="form-label">Password:</label>
                    <input
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                        required
                        className="form-control"
                    />
                </div>
                {error && <p className="text-danger text-center" style={{ color: 'red' }}>{error}</p>}
                <button type="submit" className="btn btn-primary w-100">Signup</button>
            </form>
            </div>
        </div>
    );
        
};


export default Signup;