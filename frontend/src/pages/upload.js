import React, { useState, useEffect, useRef } from 'react';
import { useRouter } from 'next/navigation';
import axios from 'axios';
import Select from 'react-select';
import { useMutation, queryClient } from 'react-query';
import CreatableSelect from 'react-select/creatable';

export default function Upload() {
  const router = useRouter();
  const uploadButtonRef = useRef(null);
  const fileInputRef = useRef(null);
  const [selectedSymptoms, setSelectedSymptoms] = useState([]);
  const [selectedMedications, setSelectedMedications] = useState([]);
  const [selectedAllergies, setSelectedAllergies] = useState([]);
  const [genderOther, setGenderOther] = useState('');
  const [raceOther, setRaceOther] = useState('');
  const [selectedGender, setSelectedGender] = useState('');
  const [selectedRace, setSelectedRace] = useState('');
  const [isClient, setIsClient] = useState(false);
  const [height, setHeight] = useState('');
  const [weight, setWeight] = useState('');
  const [age, setAge] = useState('');
  const [activeSection, setActiveSection] = useState('personal');

  // File upload mutation
  const mutation = useMutation(
    (formData) => axios.post('http://localhost:4545/api/uploadFile', formData),
    {
      onSuccess: (response) => {
        const data = response.data;
        if (data.height) setHeight(data.height);
        if (data.weight) setWeight(Number(data.weight));
        if (data.age) setAge(Number(data.age));
        if (data.gender) setSelectedGender(data.gender);
        if (data.genderOther) setGenderOther(data.genderOther);
        if (data.race) setSelectedRace(data.race);
        if (data.raceOther) setRaceOther(data.raceOther);
        if (data.symptoms) setSelectedSymptoms(data.symptoms.map(s => ({ value: s, label: s })));
        if (data.bloodpressure) document.getElementById('bloodpressure').value = data.bloodpressure;
        if (data.heartrate) document.getElementById('heartrate').value = data.heartrate;
        if (data.temperature) document.getElementById('temperature').value = data.temperature;
        if (data.medications) setSelectedMedications(data.medications.map(m => ({ value: m, label: m })));
        if (data.allergies) setSelectedAllergies(data.allergies.map(a => ({ value: a, label: a })));
        if (data.alcohol) document.querySelector(`input[name="alcoholuse"][value="${data.alcohol}"]`).checked = true;
        if (data.smoking) document.querySelector(`input[name="smoking"][value="${data.smoking}"]`).checked = true;
        if (data.druguse) document.querySelector(`input[name="druguse"][value="${data.druguse}"]`).checked = true;
        alert('Form autofilled successfully!');
      },
      onError: (error) => {
        console.error('Error uploading file or autofilling form:', error);
        alert('Error uploading file or autofilling form.');
      },
    }
  );

  useEffect(() => {
    const uploadButton = uploadButtonRef.current;
    const fileInput = fileInputRef.current;

    const handleUpload = () => {
      if (!fileInput || fileInput.files.length === 0) {
        alert('Please select a file to upload.');
        return;
      }
      const file = fileInput.files[0];
      const formData = new FormData();
      formData.append('file', file);

      mutation.mutate(formData);
    };

    if (uploadButton) {
      uploadButton.addEventListener('click', handleUpload);
    }

    return () => {
      if (uploadButton) {
        uploadButton.removeEventListener('click', handleUpload);
      }
    };
  }, [mutation]);

  useEffect(() => {
    setIsClient(true);
  }, []);

  // Options for dropdowns
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
    { value: 'sore_throat', label: 'Sore throat' }
  ];

  const medicationOptions = [
    { value: 'aspirin', label: 'Aspirin' },
    { value: 'ibuprofen', label: 'Ibuprofen' },
    { value: 'acetaminophen', label: 'Acetaminophen' },
    { value: 'lisinopril', label: 'Lisinopril' },
    { value: 'metformin', label: 'Metformin' },
    { value: 'atorvastatin', label: 'Atorvastatin' },
    { value: 'albuterol', label: 'Albuterol' },
    { value: 'omeprazole', label: 'Omeprazole' }
  ];

  const allergyOptions = [
    { value: 'penicillin', label: 'Penicillin' },
    { value: 'sulfa', label: 'Sulfa drugs' },
    { value: 'aspirin', label: 'Aspirin' },
    { value: 'ibuprofen', label: 'Ibuprofen' },
    { value: 'latex', label: 'Latex' },
    { value: 'peanuts', label: 'Peanuts' },
    { value: 'shellfish', label: 'Shellfish' },
    { value: 'eggs', label: 'Eggs' }
  ];

  const handleManual = async (e) => {
    e.preventDefault();
    try {
      const formData = {
        height: e.target.height.value ? parseInt(e.target.height.value) : 0,
        weight: e.target.weight.value ? parseInt(e.target.weight.value) : 0,
        age: e.target.age.value ? parseInt(e.target.age.value) : 0,
        gender: selectedGender === 'Other' ? genderOther : selectedGender,
        race: selectedRace === 'Other' ? raceOther : selectedRace,
        symptoms: selectedSymptoms.map(s => s.value),
        bloodpressure: e.target.bloodpressure.value ? parseInt(e.target.bloodpressure.value) : 0,
        heartrate: e.target.heartrate.value ? parseInt(e.target.heartrate.value) : 0,
        temperature: e.target.temperature.value ? parseFloat(e.target.temperature.value) : 0,
        medications: selectedMedications.map(m => m.value),
        allergies: selectedAllergies.map(a => a.value),
        alcohol: e.target.alcoholuse.value || "0",
        smoking: e.target.smoking.value || "0",
        druguse: e.target.druguse.value || "0",
      };
      const response = await axios.post('http://localhost:4545/api/uploadForm', formData, {
        headers: {
          'Content-Type': 'application/json',
        },
        timeout: 5000,
        withCredentials: true,
      });
      alert('Upload successful!');
      router.push('/loading');
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
    return null;
  }

  return (
    <div className="container-fluid py-4" style={{ backgroundColor: '#000' }}>
      <div className="row justify-content-center">
        <div className="col-lg-10">
          <div className="card shadow-sm mb-4">
            <div className="card-header bg-light">
              <h3 className="h5 mb-0">Upload Medical File</h3>
            </div>
            <div className="card-body">
              <div className="d-flex align-items-center gap-3">
                <div className="flex-grow-1">
                  <label className="form-label visually-hidden">Select PDF file to autofill form:</label>
                  <input 
                    type="file" 
                    id="fileInput" 
                    ref={fileInputRef} 
                    className="form-control" 
                    accept=".pdf" 
                  />
                </div>
                <div>
                  <button 
                    id="uploadButton" 
                    ref={uploadButtonRef} 
                    className="btn btn-primary"
                    type="button"
                    style={{ height: '38px' }}
                  >
                    <i className="bi bi-upload me-2"></i>Upload File
                  </button>
                </div>
              </div>
            </div>
          </div>

          <div className="card shadow-sm">
            <div className="card-header bg-primary text-white">
              <h1 className="h4 mb-0 text-center">Patient Medical Information Form</h1>
            </div>
            
            <div className="card-body">
              <ul className="nav nav-tabs mb-4">
                <li className="nav-item">
                  <button 
                    className={`nav-link ${activeSection === 'personal' ? 'active' : ''}`}
                    onClick={() => setActiveSection('personal')}
                  >
                    Personal Info
                  </button>
                </li>
                <li className="nav-item">
                  <button 
                    className={`nav-link ${activeSection === 'symptoms' ? 'active' : ''}`}
                    onClick={() => setActiveSection('symptoms')}
                  >
                    Symptoms
                  </button>
                </li>
                <li className="nav-item">
                  <button 
                    className={`nav-link ${activeSection === 'biometrics' ? 'active' : ''}`}
                    onClick={() => setActiveSection('biometrics')}
                  >
                    Biometrics
                  </button>
                </li>
                <li className="nav-item">
                  <button 
                    className={`nav-link ${activeSection === 'history' ? 'active' : ''}`}
                    onClick={() => setActiveSection('history')}
                  >
                    Medical History
                  </button>
                </li>
              </ul>

              <form id="medicalForm" onSubmit={handleManual}>
                <div className={`section ${activeSection === 'personal' ? 'd-block' : 'd-none'}`}>
                  <div className="mb-4">
                    <h3 className="mb-3 border-bottom pb-2">Personal Information</h3>
                    
                    <div className="row">
                      <div className="col-md-4 mb-3">
                        <label htmlFor="height" className="form-label">Height (cm):</label>
                        <input
                          type="number"
                          id="height"
                          name="height"
                          className="form-control"
                          value={height}
                          onChange={(e) => setHeight(e.target.value)}
                        />
                      </div>
                      <div className="col-md-4 mb-3">
                        <label htmlFor="weight" className="form-label">Weight (kg):</label>
                        <input
                          type="number"
                          id="weight"
                          name="weight"
                          className="form-control"
                          value={weight}
                          onChange={(e) => setWeight(e.target.value)}
                        />
                      </div>
                      <div className="col-md-4 mb-3">
                        <label htmlFor="age" className="form-label">Age:</label>
                        <input
                          type="number"
                          id="age"
                          name="age"
                          className="form-control"
                          value={age}
                          onChange={(e) => setAge(e.target.value)}
                        />
                      </div>
                    </div>
                    
                    <div className="row">
                      <div className="col-md-6 mb-3">
                        <label className="form-label">Gender:</label>
                        <div className="form-check">
                          <input 
                            className="form-check-input" 
                            type="radio" 
                            name="gender" 
                            id="genderMale" 
                            value="Male" 
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
                        />
                      </div>
                      
                      <div className="col-md-6 mb-3">
                        <label className="form-label">Race/Ethnicity:</label>
                        <div className="form-check">
                          <input 
                            className="form-check-input" 
                            type="radio" 
                            name="race" 
                            id="raceWhite" 
                            value="White" 
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
                          placeholder="Specify race/ethnicity"
                          value={raceOther}
                          onChange={(e) => setRaceOther(e.target.value)}
                          disabled={selectedRace !== 'Other'}
                        />
                      </div>
                    </div>
                  </div>
                </div>

                <div className={`section ${activeSection === 'symptoms' ? 'd-block' : 'd-none'}`}>
                  <div className="mb-4">
                    <h3 className="mb-3 border-bottom pb-2">Symptoms</h3>
                    <div className="mb-3">
                      <label htmlFor="symptoms" className="form-label">Current Symptoms:</label>
                      <CreatableSelect
                        isMulti
                        name="symptoms"
                        options={symptomOptions}
                        className="basic-multi-select"
                        classNamePrefix="select"
                        onChange={setSelectedSymptoms}
                        value={selectedSymptoms}
                        placeholder="Select or type symptoms..."
                        required
                      />
                    </div>
                  </div>
                </div>

  
                <div className={`section ${activeSection === 'biometrics' ? 'd-block' : 'd-none'}`}>
                  <div className="mb-4">
                    <h3 className="mb-3 border-bottom pb-2">Biometric Information</h3>
                    <div className="row">
                      <div className="col-md-4 mb-3">
                        <label htmlFor="bloodpressure" className="form-label">Blood Pressure (mmHg):</label>
                        <input 
                          type="number" 
                          id="bloodpressure" 
                          name="bloodpressure" 
                          className="form-control" 
                        />
                      </div>
                      <div className="col-md-4 mb-3">
                        <label htmlFor="heartrate" className="form-label">Heart Rate (bpm):</label>
                        <input 
                          type="number" 
                          id="heartrate" 
                          name="heartrate" 
                          className="form-control" 
                        />
                      </div>
                      <div className="col-md-4 mb-3">
                        <label htmlFor="temperature" className="form-label">Temperature (°C):</label>
                        <input 
                          type="number" 
                          id="temperature" 
                          name="temperature" 
                          className="form-control" 
                          step="0.1" 
                        />
                      </div>
                    </div>
                  </div>
                </div>

                <div className={`section ${activeSection === 'history' ? 'd-block' : 'd-none'}`}>
                  <div className="mb-4">
                    <h3 className="mb-3 border-bottom pb-2">Medical History</h3>
                    
                    <div className="row">
                      <div className="col-md-6 mb-3">
                        <label htmlFor="medications" className="form-label">Current Medications:</label>
                        <CreatableSelect
                          isMulti
                          name="medications"
                          options={medicationOptions}
                          className="basic-multi-select"
                          classNamePrefix="select"
                          onChange={setSelectedMedications}
                          value={selectedMedications}
                          placeholder="Select or type medications..."
                        />
                      </div>
                      
                      <div className="col-md-6 mb-3">
                        <label htmlFor="allergies" className="form-label">Known Allergies:</label>
                        <CreatableSelect
                          isMulti
                          name="allergies"
                          options={allergyOptions}
                          className="basic-multi-select"
                          classNamePrefix="select"
                          onChange={setSelectedAllergies}
                          value={selectedAllergies}
                          placeholder="Select or type allergies..."
                        />
                      </div>
                    </div>
                    
                    <div className="row">
                      <div className="col-md-4 mb-3">
                        <label className="form-label">Alcohol Use:</label>
                        <div className="form-check">
                          <input className="form-check-input" type="radio" name="alcoholuse" id="alcoholNever" value="Never" />
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
                      </div>
                      
                      <div className="col-md-4 mb-3">
                        <label className="form-label">Smoking:</label>
                        <div className="form-check">
                          <input className="form-check-input" type="radio" name="smoking" id="smokingNever" value="Never" />
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
                      </div>
                      
                      <div className="col-md-4 mb-3">
                        <label className="form-label">Drug Use:</label>
                        <div className="form-check">
                          <input className="form-check-input" type="radio" name="druguse" id="druguseNever" value="Never" />
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
                      </div>
                    </div>
                  </div>
                </div>
              </form>

              <div className="d-flex justify-content-between mt-4">
                <button 
                  type="button" 
                  className="btn btn-outline-primary"
                  onClick={() => {
                    if (activeSection === 'personal') return;
                    if (activeSection === 'symptoms') setActiveSection('personal');
                    if (activeSection === 'biometrics') setActiveSection('symptoms');
                    if (activeSection === 'history') setActiveSection('biometrics');
                  }}
                  disabled={activeSection === 'personal'}
                >
                  Previous
                </button>
                
                {activeSection !== 'history' ? (
                  <button 
                    type="button" 
                    className="btn btn-primary"
                    onClick={() => {
                      if (activeSection === 'symptoms' && selectedSymptoms.length === 0) {
                        alert('Please select at least one symptom');
                        return;
                      }
                      if (activeSection === 'personal') setActiveSection('symptoms');
                      if (activeSection === 'symptoms') setActiveSection('biometrics');
                      if (activeSection === 'biometrics') setActiveSection('history');
                    }}
                  >
                    Next
                  </button>
                ) : (
                  <button 
                    type="submit" 
                    form="medicalForm" 
                    className="btn btn-success"
                    disabled={activeSection === 'history'}
                  >
                    Submit Form
                  </button>
                )}
                </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}