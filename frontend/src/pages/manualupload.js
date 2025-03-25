import React from 'react';
import { useRouter } from 'next/navigation';
import axios from 'axios';

export default function ManualUpload() {
  const router = useRouter();
  const handleManual = async (e) => {
    e.preventDefault();
    try {
        const response = await axios.post('http://localhost:4545/api/manualupload', {
            physicalinfo: e.target.physicalinfo.value,
            symptoms: e.target.symptoms.value,
            biometricinfo: e.target.biometricinfo.value,
            medicalhistory: e.target.medicalhistory.value,
        }, {
            headers: {
                'Content-Type': 'application/json',
            },
        });
        alert('Upload successful!');
        // Redirect to the home page
        router.push('/');
    }
    catch (err) {
        console.error('Error uploading data:', err);
        alert('Error uploading data.');
    }
};
  // Render the upload form
  // CHANGE THESE BASED ON THE ACTUAL ENTRY VALUES THE USER NEEDS TO INPUT
    return (
    <div className="d-flex justify-content-center vh-100">
      <div>
      <h1 className="text-center mb-3">Upload</h1>
      <div className="d-flex align-items-center justify-content-center">
      <form onSubmit={handleManual}>
          <label htmlFor="patientname" className="form-label">Patient Name:</label><br></br>
          <input type="text" id="patientname" name="patientname" className="form-control"></input><br></br>
          <label htmlFor="symptoms" className="form-label">Symptoms:</label><br></br>
          <input type="text" id="symptoms" name="symptoms" className="form-control"></input><br></br>
          <label htmlFor="treatmentplan" className="form-label">Treatment Plan:</label><br></br>
          <input type="text" id="treatmentplan" name="treatmentplan" className="form-control"></input><br></br>
          <label htmlFor="drugusageplan" className="form-label">Drug Usage Plan:</label><br></br>
          <input type="text" id="drugusageplan" name="drugusageplan" className="form-control"></input><br></br>
          <input type="submit" value="Submit" className="btn btn-primary w-100"></input>
      </form>
      </div>
      </div>
      
    </div>)
  }