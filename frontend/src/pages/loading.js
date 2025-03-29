import React, { useEffect, useState } from 'react';
import './loading.css'; // Optional: Add custom styles for the loading page

const Loading = () => {
    const [isFinished, setIsFinished] = useState(false);

    useEffect(() => {
        const checkApiStatus = async () => {
            try {
                const response = await fetch('https://0.0.0.0:4545/api/status'); // Replace with your API endpoint
                const data = await response.json();
                if (data.finished) {
                    setIsFinished(true);
                }
            } catch (error) {
                console.error('Error checking API status:', error);
            }
        };

        const interval = setInterval(checkApiStatus, 5000); // Poll every 5 seconds

        return () => clearInterval(interval); // Cleanup on component unmount
    }, []);

    if (isFinished) {
        return <p>Process is complete!</p>; // Replace with your desired UI when finished
    }

    return (
        <div className="loading-container">
            <div className="spinner"></div>
            <p>Loading, please wait...</p>
        </div>
    );
};

export default Loading;