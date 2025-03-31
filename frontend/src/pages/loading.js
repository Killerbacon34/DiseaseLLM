import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useRouter } from 'next/navigation';


const Loading = () => {
    const [isFinished, setIsFinished] = useState(false);
    const Router = useRouter();
    useEffect(() => {
        const checkApiStatus = async () => {
            try {
                const response = await axios.get('http://localhost:4545/api/status', {
                    withCredentials: true,
                });
                console.log(response.data); 
                if (response.data === 'true') {
                    setIsFinished(true); //TO-DO: FIX 
                    console.log("triggered")
                }
            } catch (error) {
                console.error('Error checking API status:', error);
            } 
        };

        const interval = setInterval(checkApiStatus, 5000); // Poll every 5 seconds

        return () => clearInterval(interval); // Cleanup on component unmount
    }, []);

    if (isFinished) {
        console.log("triggered1")
        Router.push('/diagnosis'); 
    }

    return (
        <div className="loading-container">
            <div className="spinner"></div>
            <p>Loading, please wait...</p>
        </div>
    );
};

export default Loading;