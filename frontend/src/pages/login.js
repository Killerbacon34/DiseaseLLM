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
            redirect('/upload');
        } catch (err) {
            setError('Invalid username or password');
            console.log(err);
        }
    };

    return (
        <div>
            <h2>Login</h2>
            <form onSubmit={handleLogin}>
                <div>
                    <label>Username:</label>
                    <input
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        required
                    />
                </div>
                <div>
                    <label>Password:</label>
                    <input
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                        required
                    />
                </div>
                {error && <p style={{ color: 'red' }}>{error}</p>}
                <button type="submit">LOGIN</button>
            </form>
            
        </div>
    );
        
};


export default login;