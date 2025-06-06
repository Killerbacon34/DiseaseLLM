import React, { useEffect, useState } from "react";
import { motion } from "framer-motion";
import axios from "axios";
 import ReactMarkdown from "react-markdown";
 import rehypeKatex from "rehype-katex";
 import remarkMath from "remark-math";
 import { useRouter } from 'next/navigation';

const customStyles = `
  body {
    background: linear-gradient(to bottom, #000000, #00001f);
    color: #fff;
    min-height: 100vh;
    padding: 2rem 0;
  }
  
  .diagnosis-card {
    background-color: rgba(40, 44, 52, 0.8);
    border-radius: 15px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    margin-bottom: 1.5rem;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }
  
  .diagnosis-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  }
  
  .card-header {
    background-color: rgba(59, 130, 246, 0.2);
    border-bottom: 1px solid rgba(59, 130, 246, 0.3);
    border-top-left-radius: 15px !important;
    border-top-right-radius: 15px !important;
    color: #78aeff;
    font-weight: 500;
    font-size: 1.2rem;
  }
  
  .home-button {
    background-color: #3b82f6;
    border-radius: 50px;
    padding: 0.75rem 1.5rem;
    color: white;
    border: none;
    box-shadow: 0 4px 10px rgba(59, 130, 246, 0.3);
    transition: all 0.3s ease;
  }
  
  .home-button:hover {
    background-color: #2563eb;
    box-shadow: 0 6px 14px rgba(59, 130, 246, 0.4);
    transform: translateY(-2px);
  }
  
  .page-title {
    font-weight: 500;
    margin-bottom: 2rem;
  }
`;

const mockData = {
  "Diagnosis": "Error: Not Found, Input Data Again.",
  "Drug Usage Plan": "Error: Not Found, Input Data Again.",
  "Treatment Plan": "Error: Not Found, Input Data Again."
};

const InfoCards = ({ data }) => {
  return (
    <div className="row justify-content-center">
      <div className="col-md-10 col-lg-8">
        {Object.entries(data).map(([key, value], index) => (
          <motion.div
            key={key}
            className="diagnosis-card card"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: index * 0.2 }}
            whileHover={{ y: -5 }}
          >
            <div className="card-header">
              {key}
            </div>
            <div className="card-body">
              <div className="card-text">
                  <ReactMarkdown 
                    remarkPlugins={[remarkMath]}
                    rehypePlugins={[rehypeKatex]}
                  >
                    {String(value)}
                  </ReactMarkdown>
                </div>
            </div>
          </motion.div>
        ))}
      </div>
    </div>
  );
};

const Diagnosis = () => {
  const [jsonData, setJsonData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Simulate data loading with a timeout
    const timer = setTimeout(() => {
    axios.get("https://backend-service-646481361829.us-central1.run.app/anon/results", {
         withCredentials: true,
         headers: {
             "Accept": "application/json",
         },
     }) // Replace with actual API URL
       .then((response) => {
         setJsonData(response.data);
         setLoading(false);
       })
       .catch((err) => {
         setError(err.message);
         setLoading(false);
       });
      
    }, 1000);
    
    return () => clearTimeout(timer);
  }, []);

  const goToHome = () => {
    // In a real app with router: router.push("/")
    console.log("Navigate to home");
    window.location.href = "/";
  };

  const goToUpload = () => {
    // In a real app with router: router.push("/")
    console.log("Navigate to upload");
    window.location.href = "/upload";
  };

  return (
    <div className="container">
      {/* Add the custom styles */}
      <style>{customStyles}</style>
      
      <h1 className="text-center page-title">Your Health Summary</h1>
      
      {loading && (
        <div className="text-center my-5">
          <div className="spinner-border text-primary" role="status">
            <span className="visually-hidden">Loading...</span>
          </div>
        </div>
      )}
      
      {error && (
        <div className="alert alert-danger mx-auto" style={{ maxWidth: "600px" }}>
          <p className="mb-0">{error}</p>
        </div>
      )}
      
      {jsonData && <InfoCards data={jsonData} />}
      
      <div className="text-center mt-4 mb-5">
        <button onClick={goToHome} className="home-button btn">
          <span className="me-2">←</span> Return Home
        </button>
      </div>
      <div className="text-center mt-4 mb-5">
        <button onClick={goToUpload} className="home-button btn">
          <span className="me-2">←</span> Try Different Inputs
        </button>
      </div>
    </div>
  );
};

export default Diagnosis;