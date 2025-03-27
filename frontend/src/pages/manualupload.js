import React from 'react';
import { useRouter } from 'next/navigation';
import axios from 'axios';

export default function ManualUpload() {
  const router = useRouter();
  const handleManual = async (e) => {
    e.preventDefault();
    try {
        const response = await axios.post('http://localhost:4545/api/manualupload', {
            height: e.target.height.value,
            weight: e.target.weight.value,
            age: e.target.age.value,
            gender: e.target.gender.value,
            race: e.target.race.value,
            symptoms: e.target.symptoms.value,
            bloodpressure: e.target.bloodpressure.value,
            heartRate: e.target.heartRate.value,
            temperature: e.target.temperature.value,
            medications: e.target.medications.value,
            allergies: e.target.allergies.value,
            alcoholuse: e.target.alcoholuse.value,
            smoking: e.target.smoking.value,
            druguse: e.target.druguse.value,
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
      <label htmlFor="height" className="form-label">Height:</label><br></br>
      <input type="text" id="height" name="height" className="form-control"></input><br></br>
      <label htmlFor="weight" className="form-label">Weight:</label><br></br>
      <input type="text" id="weight" name="weight" className="form-control"></input><br></br>
      <label htmlFor="age" className="form-label">Age:</label><br></br>
      <input type="text" id="age" name="age" className="form-control"></input><br></br>
      <label htmlFor="gender" className="form-label">Gender:</label><br></br>
      <input type="text" id="gender" name="gender" className="form-control"></input><br></br>
      <label htmlFor="race" className="form-label">Race:</label><br></br>
      <input type="text" id="race" name="race" className="form-control"></input><br></br>
      <label htmlFor="symptoms" className="form-label">Symptoms:</label><br></br>
      <input type="text" id="symptoms" name="symptoms" className="form-control"></input><br></br>
      <label htmlFor="bloodpressure" className="form-label">Blood Pressure:</label><br></br>
      <input type="text" id="bloodpressure" name="bloodpressure" className="form-control"></input><br></br>
      <label htmlFor="heartRate" className="form-label">Heart Rate:</label><br></br>
      <input type="text" id="heartRate" name="heartRate" className="form-control"></input><br></br>
      <label htmlFor="temperature" className="form-label">Temperature:</label><br></br>
      <input type="text" id="temperature" name="temperature" className="form-control"></input><br></br>
      <label htmlFor="medications" className="form-label">Medications:</label><br></br>
      <input type="text" id="medications" name="medications" className="form-control"></input><br></br>
      <label htmlFor="allergies" className="form-label">Allergies:</label><br></br>
      <input type="text" id="allergies" name="allergies" className="form-control"></input><br></br>
      <label htmlFor="alcoholuse" className="form-label">Alcohol Use:</label><br></br>
      <input type="text" id="alcoholuse" name="alcoholuse" className="form-control"></input><br></br>
      <label htmlFor="smoking" className="form-label">Smoking:</label><br></br>
      <input type="text" id="smoking" name="smoking" className="form-control"></input><br></br>
      <label htmlFor="druguse" className="form-label">Drug Use:</label><br></br>
      <input type="text" id="druguse" name="druguse" className="form-control"></input><br></br>
      <input type="submit" value="Submit" className="btn btn-primary w-100"></input>
      </form>
      </div>
      </div>
      
    </div>)
  }