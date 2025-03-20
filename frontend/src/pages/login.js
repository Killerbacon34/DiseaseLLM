import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Fingerprint2 from 'fingerprintjs2';
import { useRouter } from 'next/navigation';
import { redirect } from 'next/navigation';

const login = () => {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const [deviceId, setDeviceId] = useState('');
    const router = useRouter();

    useEffect(() => {
        if (window.requestIdleCallback) {
            requestIdleCallback(() => {
                Fingerprint2.get((components) => {
                    const values = components.map(component => component.value);
                    const murmur = Fingerprint2.x64hash128(values.join(''), 31);
                    setDeviceId(murmur);
                    console.log('Device ID:', murmur);
                });
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

    const handleLogin = async (e) => {
        e.preventDefault();
        try {
            const response = await axios.post('https://backend-service-yzqvkugauq-uc.a.run.app/api/login', {
                username: username,
                pass: password,
                devid: deviceId,
            }, {
                headers: {
                    'Content-Type': 'application/json',
                },
            });
            const token = response.data;
            sessionStorage.setItem('Auth', token);
            alert('Login successful!');
            setError('');
            router.push('/upload');
        } catch (err) {
            setError('Invalid username or password');
            console.log(err);
        }
    };
return (
    <div className="d-flex align-items-center justify-content-center vh-100">
        <div>
            <h2 className="text-center mb-3">Login</h2>
            <p className="text-center mt-3">
                <small>or <span 
                    className="text-primary cursor-pointer" 
                    style={{ cursor: "pointer" }}
                    onClick={() => router.push('/signup')}
                >create an account</span></small>
            </p>
            <form onSubmit={handleLogin}>
                <div className="mb-3">
                    <label className="form-label">Username</label>
                    <input
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        required
                        className="form-control"
                    />
                </div>
                <div className="mb-3">
                    <label className="form-label">Password</label>
                    <input
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                        required
                        className="form-control"
                    />
                </div>
                {error && <p className="text-danger text-center">{error}</p>}
                <button
                    type="submit"
                    className="btn btn-primary w-100"
                >
                    LOGIN
                </button>
            </form>
            
        </div>
    </div>
);
};

export default login;
