import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import axios from 'axios';
import Select from 'react-select';

export default function ManualUpload() {
  const router = useRouter();
  const [selectedSymptoms, setSelectedSymptoms] = useState([]);
  const [selectedMedications, setSelectedMedications] = useState([]);
  const [selectedAllergies, setSelectedAllergies] = useState([]);
  const [genderOther, setGenderOther] = useState('');
  const [raceOther, setRaceOther] = useState('');
  const [selectedGender, setSelectedGender] = useState('');
  const [selectedRace, setSelectedRace] = useState('');
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  const symptomOptions = [
    { value: 'headache', label: 'Headache' },
    { value: 'fever', label: 'Fever' },
    { value: 'cough', label: 'Cough' },
    { value: 'fatigue', label: 'Fatigue' },
    { value: 'shortness_of_breath', label: 'Shortness of breath' },
    { value: 'chest_pain', label: 'Chest pain' },
    { value: 'nausea', label: 'Nausea' },
    { value: 'dizziness', label: 'Dizziness' },
    { value: 'muscle_pain', label: 'Muscle pain' },
    { value: 'sore_throat', label: 'Sore throat' },
    { value: 'other', label: 'Other' }
  ];

  const medicationOptions = [
    { value: 'aspirin', label: 'Aspirin' },
    { value: 'ibuprofen', label: 'Ibuprofen' },
    { value: 'acetaminophen', label: 'Acetaminophen' },
    { value: 'lisinopril', label: 'Lisinopril' },
    { value: 'metformin', label: 'Metformin' },
    { value: 'atorvastatin', label: 'Atorvastatin' },
    { value: 'albuterol', label: 'Albuterol' },
    { value: 'omeprazole', label: 'Omeprazole' },
    { value: 'other', label: 'Other' }
  ];

  const allergyOptions = [
    { value: 'penicillin', label: 'Penicillin' },
    { value: 'sulfa', label: 'Sulfa drugs' },
    { value: 'aspirin', label: 'Aspirin' },
    { value: 'ibuprofen', label: 'Ibuprofen' },
    { value: 'latex', label: 'Latex' },
    { value: 'peanuts', label: 'Peanuts' },
    { value: 'shellfish', label: 'Shellfish' },
    { value: 'eggs', label: 'Eggs' },
    { value: 'other', label: 'Other' }
  ];

  const handleManual = async (e) => {
    e.preventDefault();
    try {
      const formData = {
        height: e.target.height.value,
        weight: e.target.weight.value,
        age: e.target.age.value,
        gender: selectedGender === 'Other' ? genderOther : selectedGender,
        race: selectedRace === 'Other' ? raceOther : selectedRace,
        symptoms: selectedSymptoms.map(s => s.value),
        bloodpressure: e.target.bloodpressure.value,
        heartRate: e.target.heartRate.value,
        temperature: e.target.temperature.value,
        medications: selectedMedications.map(m => m.value),
        allergies: selectedAllergies.map(a => a.value),
        alcoholuse: e.target.alcoholuse.value,
        smoking: e.target.smoking.value,
        druguse: e.target.druguse.value,
      };

      const response = await axios.post('http://localhost:4545/api/manualupload', formData, {
        headers: {
          'Content-Type': 'application/json',
        },
      });
      alert('Upload successful!');
      router.push('/');
    } catch (err) {
      console.error('Error uploading data:', err);
      alert('Error uploading data.');
    }
  };

  const handleGenderChange = (e) => {
    setSelectedGender(e.target.value);
  };

  const handleRaceChange = (e) => {
    setSelectedRace(e.target.value);
  };

  if (!isClient) {
    return null; // or return a loading spinner
  }

  return (
    <div className="d-flex justify-content-center vh-100">
      <div>
        <h1 className="text-center mb-3">Upload</h1>
        <div className="d-flex align-items-center justify-content-center">
          <form onSubmit={handleManual}>
            
            {/* Personal Information Section */}
            <h3 className="mt-4 mb-3">Personal Information</h3>
            <label htmlFor="height" className="form-label">Height (cm):</label><br></br>
            <input type="number" id="height" name="height" className="form-control" required></input><br></br>
            <label htmlFor="weight" className="form-label">Weight (kg):</label><br></br>
            <input type="number" id="weight" name="weight" className="form-control" required></input><br></br>
            <label htmlFor="age" className="form-label">Age:</label><br></br>
            <input type="number" id="age" name="age" className="form-control" required></input><br></br>
            
            <label className="form-label">Gender:</label><br></br>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="gender" 
                id="genderMale" 
                value="Male" 
                required 
                checked={selectedGender === 'Male'}
                onChange={handleGenderChange}
              />
              <label className="form-check-label" htmlFor="genderMale">Male</label>
            </div>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="gender" 
                id="genderFemale" 
                value="Female" 
                checked={selectedGender === 'Female'}
                onChange={handleGenderChange}
              />
              <label className="form-check-label" htmlFor="genderFemale">Female</label>
            </div>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="gender" 
                id="genderOther" 
                value="Other" 
                checked={selectedGender === 'Other'}
                onChange={handleGenderChange}
              />
              <label className="form-check-label" htmlFor="genderOther">Other</label>
            </div>
            <input 
              type="text" 
              id="genderOtherText" 
              name="genderOtherText" 
              className="form-control mt-2" 
              placeholder="Specify gender"
              value={genderOther}
              onChange={(e) => setGenderOther(e.target.value)}
              disabled={selectedGender !== 'Other'}
            ></input><br></br>
            
            <label className="form-label">Race:</label><br></br>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="race" 
                id="raceWhite" 
                value="White" 
                required 
                checked={selectedRace === 'White'}
                onChange={handleRaceChange}
              />
              <label className="form-check-label" htmlFor="raceWhite">White</label>
            </div>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="race" 
                id="raceBlack" 
                value="Black" 
                checked={selectedRace === 'Black'}
                onChange={handleRaceChange}
              />
              <label className="form-check-label" htmlFor="raceBlack">Black/African American</label>
            </div>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="race" 
                id="raceAsian" 
                value="Asian" 
                checked={selectedRace === 'Asian'}
                onChange={handleRaceChange}
              />
              <label className="form-check-label" htmlFor="raceAsian">Asian</label>
            </div>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="race" 
                id="raceHispanic" 
                value="Hispanic" 
                checked={selectedRace === 'Hispanic'}
                onChange={handleRaceChange}
              />
              <label className="form-check-label" htmlFor="raceHispanic">Hispanic/Latino</label>
            </div>
            <div className="form-check">
              <input 
                className="form-check-input" 
                type="radio" 
                name="race" 
                id="raceOther" 
                value="Other" 
                checked={selectedRace === 'Other'}
                onChange={handleRaceChange}
              />
              <label className="form-check-label" htmlFor="raceOther">Other</label>
            </div>
            <input 
              type="text" 
              id="raceOtherText" 
              name="raceOtherText" 
              className="form-control mt-2" 
              placeholder="Specify race"
              value={raceOther}
              onChange={(e) => setRaceOther(e.target.value)}
              disabled={selectedRace !== 'Other'}
            ></input><br></br>
            
            {/* Symptoms Section */}
            <h3 className="mt-4 mb-3">Symptoms</h3>
            <label htmlFor="symptoms" className="form-label">Symptoms:</label><br></br>
            <Select
              isMulti
              name="symptoms"
              options={symptomOptions}
              className="basic-multi-select"
              classNamePrefix="select"
              onChange={setSelectedSymptoms}
              value={selectedSymptoms}
              required
            />
            <br></br>
            
            {/* Biometric Information Section */}
            <h3 className="mt-4 mb-3">Biometric Information</h3>
            <label htmlFor="bloodpressure" className="form-label">Blood Pressure (mmHg):</label><br></br>
            <input type="number" id="bloodpressure" name="bloodpressure" className="form-control" required></input><br></br>
            <label htmlFor="heartRate" className="form-label">Heart Rate (bpm):</label><br></br>
            <input type="number" id="heartRate" name="heartRate" className="form-control" required></input><br></br>
            <label htmlFor="temperature" className="form-label">Temperature (°C):</label><br></br>
            <input type="number" id="temperature" name="temperature" className="form-control" step="0.1" required></input><br></br>
            
            {/* Medical History Section */}
            <h3 className="mt-4 mb-3">Medical History</h3>
            <label htmlFor="medications" className="form-label">Medications:</label><br></br>
            <Select
              isMulti
              name="medications"
              options={medicationOptions}
              className="basic-multi-select"
              classNamePrefix="select"
              onChange={setSelectedMedications}
              value={selectedMedications}
            />
            <br></br>
            
            <label htmlFor="allergies" className="form-label">Allergies:</label><br></br>
            <Select
              isMulti
              name="allergies"
              options={allergyOptions}
              className="basic-multi-select"
              classNamePrefix="select"
              onChange={setSelectedAllergies}
              value={selectedAllergies}
            />
            <br></br>
            
            <label className="form-label">Alcohol Use:</label><br></br>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="alcoholuse" id="alcoholNever" value="Never" required />
              <label className="form-check-label" htmlFor="alcoholNever">Never</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="alcoholuse" id="alcoholRarely" value="Rarely" />
              <label className="form-check-label" htmlFor="alcoholRarely">Rarely</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="alcoholuse" id="alcoholMonthly" value="Monthly" />
              <label className="form-check-label" htmlFor="alcoholMonthly">Monthly</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="alcoholuse" id="alcoholWeekly" value="Weekly" />
              <label className="form-check-label" htmlFor="alcoholWeekly">Weekly</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="alcoholuse" id="alcoholDaily" value="Daily" />
              <label className="form-check-label" htmlFor="alcoholDaily">Daily</label>
            </div>
            
            <label className="form-label mt-2">Smoking:</label><br></br>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="smoking" id="smokingNever" value="Never" required />
              <label className="form-check-label" htmlFor="smokingNever">Never</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="smoking" id="smokingRarely" value="Rarely" />
              <label className="form-check-label" htmlFor="smokingRarely">Rarely</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="smoking" id="smokingMonthly" value="Monthly" />
              <label className="form-check-label" htmlFor="smokingMonthly">Monthly</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="smoking" id="smokingWeekly" value="Weekly" />
              <label className="form-check-label" htmlFor="smokingWeekly">Weekly</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="smoking" id="smokingDaily" value="Daily" />
              <label className="form-check-label" htmlFor="smokingDaily">Daily</label>
            </div>
            
            <label className="form-label mt-2">Drug Use:</label><br></br>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="druguse" id="druguseNever" value="Never" required />
              <label className="form-check-label" htmlFor="druguseNever">Never</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="druguse" id="druguseRarely" value="Rarely" />
              <label className="form-check-label" htmlFor="druguseRarely">Rarely</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="druguse" id="druguseMonthly" value="Monthly" />
              <label className="form-check-label" htmlFor="druguseMonthly">Monthly</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="druguse" id="druguseWeekly" value="Weekly" />
              <label className="form-check-label" htmlFor="druguseWeekly">Weekly</label>
            </div>
            <div className="form-check">
              <input className="form-check-input" type="radio" name="druguse" id="druguseDaily" value="Daily" />
              <label className="form-check-label" htmlFor="druguseDaily">Daily</label>
            </div>
            
            <input type="submit" value="Submit" className="btn btn-primary w-100 mt-4"></input>
          </form>
        </div>
      </div>
    </div>
  );
}