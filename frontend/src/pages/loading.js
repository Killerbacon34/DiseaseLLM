import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useRouter } from 'next/navigation';

const Loading = () => {
  const router = useRouter();
  const [progress, setProgress] = useState(0);
  const [loading, setLoading] = useState(0);

  useEffect(() => {
    const checkApiStatus = async () => {
      try {
        // const response = await axios.get('http://localhost:4545/api/status', {
        const response = await axios.get('https://backend-service-646481361829.us-central1.run.app/api/status', {
          withCredentials: true,
        });

        if (response.status === 200) {
          console.log("API ready, redirecting...");
          setProgress(100);
          router.push("/result");
        } else {
          console.log(response.data);
          setLoading(parseInt(response.data, 10));
        }
      } catch (error) {
        console.error('Error checking API status:', error);
      }
    };

    const interval = setInterval(() => {
      checkApiStatus();

      // Use a functional update to ensure the latest value of `loading` is used
      setProgress((prevProgress) => Math.min((25 * loading) + 10, 100));
    }, 5000);

    return () => clearInterval(interval);
  }, [router, loading]); // Add `loading` to the dependency array

  return (
    <div className="container text-center mt-5">
      <h1>Loading, please wait...</h1>
      <div className="progress mt-4" style={{ height: '30px' }}>
        <div
          className="progress-bar progress-bar-striped progress-bar-animated bg-primary"
          role="progressbar"
          style={{ width: `${progress}%` }}
          aria-valuenow={progress}
          aria-valuemin="0"
          aria-valuemax="100"
        >
          {progress}%
        </div>
      </div>
    </div>
  );
};

export default Loading;
