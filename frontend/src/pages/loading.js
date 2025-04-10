import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useRouter } from 'next/navigation';


const Loading = () => {
    const Router = useRouter();
    useEffect(() => {
        const checkApiStatus = async () => {
            try {
                const response = await axios.get('http://localhost:4545/api/status', {
                    withCredentials: true,
                });
                console.log(response); 
                if (response.status === 200) {
                    console.log("triggered")
                    Router.push("/result")
                }
            } catch (error) {
                console.error('Error checking API status:', error);
            } 
        };

        const interval = setInterval(checkApiStatus, 5000); // Poll every 5 seconds

        return () => clearInterval(interval); // Cleanup on component unmount
    }, []);

    return (
        <div className="loading-container">
            <div className="spinner"></div>
            <p>Loading, please wait...</p>
        </div>
    );
};

export default Loading;