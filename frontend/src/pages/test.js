import React, { useState } from 'react';
import axios from 'axios';
import { withCoalescedInvoke } from 'next/dist/lib/coalesced-function';
const TestApiPage = () => {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    const fetchData = async () => {
        setLoading(true);
        setError(null);
        try {
            const response = await axios.get('http://localhost:4545/anon/release', {
                withCredentials: true,
            });
            if (response.status !== 200) {
                throw new Error('Failed to fetch data');
            }
            console.log(response.data); // Log the response data to the console
        }
        catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
        try {
            const response = await axios.get('http://localhost:4545/anon/check-session', {
                withCredentials: true,
            });
            if (response.status !== 200) {
                throw new Error('Failed to fetch data');
            }
            setData(response.data); 
        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div style={{ padding: '20px', fontFamily: 'Arial, sans-serif' }}>
            <h1>API Test Page</h1>
            <button onClick={fetchData} style={{ padding: '10px 20px', cursor: 'pointer' }}>
                Test API
            </button>
            {loading && <p>Loading...</p>}
            {error && <p style={{ color: 'red' }}>Error: {error}</p>}
            {data && (
                <div style={{ marginTop: '20px' }}>
                    <h2>API Response:</h2>
                    <pre style={{ background: '#f4f4f4', padding: '10px', borderRadius: '5px' }}>
                        {JSON.stringify(data, null, 2)}
                    </pre>
                </div>
            )}
        </div>
    );
};

export default TestApiPage;