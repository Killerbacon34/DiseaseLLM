import React, { useEffect, useState } from "react";
import axios from "axios";

const JsonTable = ({ data }) => {
  return (
    <div className="p-4">
      <table className="table-auto w-full border-collapse border border-gray-300">
        <thead>
          <tr className="bg-gray-100">
            <th className="border border-gray-300 px-4 py-2">Key</th>
            <th className="border border-gray-300 px-4 py-2">Value</th>
          </tr>
        </thead>
        <tbody>
          {Object.entries(data).map(([key, value]) => (
            <tr key={key} className="hover:bg-gray-50">
              <td className="border border-gray-300 px-4 py-2 font-semibold">{key}</td>
              <td className="border border-gray-300 px-4 py-2 text-left">{value}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

const diagnosis = () => {
  const [jsonData, setJsonData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    axios.get("http://localhost:4545/anon/results", {
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
  }, []);

  if (loading) return <p className="text-center p-4">Loading...</p>;
  if (error) return <p className="text-center p-4 text-red-500">Error: {error}</p>;

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-2xl font-bold mb-4">Generated Diagnosis</h1>
      {jsonData ? <JsonTable data={jsonData} /> : <p>No data available</p>}
    </div>
  );
};

export default diagnosis; 