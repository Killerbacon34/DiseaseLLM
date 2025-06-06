import React, { useState, useEffect, useRef } from 'react';
import { useRouter } from 'next/navigation';
import axios from 'axios';
import Select from 'react-select';
import { useMutation, queryClient } from 'react-query';
import CreatableSelect from 'react-select/creatable';

// Helper function to save form state to localStorage
const saveFormState = (state) => {
  try {
    localStorage.setItem('medicalFormState', JSON.stringify(state));
  } catch (error) {
    console.error('Error saving form state:', error);
  }
};

// Helper function to load form state from localStorage
const loadFormState = () => {
  try {
    const savedState = localStorage.getItem('medicalFormState');
    return savedState ? JSON.parse(savedState) : null;
  } catch (error) {
    console.error('Error loading form state:', error);
    return null;
  }
};

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
  const [activeSection, setActiveSection] = useState('symptoms');
  const [bloodPressure, setBloodPressure] = useState('');
  const [heartRate, setHeartRate] = useState('');
  const [temperature, setTemperature] = useState('');
  const [alcoholUse, setAlcoholUse] = useState('');
  const [smoking, setSmoking] = useState('');
  const [drugUse, setDrugUse] = useState('');

  // Load saved form state on component mount
  useEffect(() => {
    const savedState = loadFormState();
    if (savedState) {
      setSelectedSymptoms(savedState.selectedSymptoms || []);
      setSelectedMedications(savedState.selectedMedications || []);
      setSelectedAllergies(savedState.selectedAllergies || []);
      setGenderOther(savedState.genderOther || '');
      setRaceOther(savedState.raceOther || '');
      setSelectedGender(savedState.selectedGender || '');
      setSelectedRace(savedState.selectedRace || '');
      setHeight(savedState.height || '');
      setWeight(savedState.weight || '');
      setAge(savedState.age || '');
      setBloodPressure(savedState.bloodPressure || '');
      setHeartRate(savedState.heartRate || '');
      setTemperature(savedState.temperature || '');
      setAlcoholUse(savedState.alcoholUse || '');
      setSmoking(savedState.smoking || '');
      setDrugUse(savedState.drugUse || '');
      setActiveSection(savedState.activeSection || 'symptoms');
    }
    setIsClient(true);
  }, []);

  // Save form state whenever it changes
  useEffect(() => {
    const formState = {
      selectedSymptoms,
      selectedMedications,
      selectedAllergies,
      genderOther,
      raceOther,
      selectedGender,
      selectedRace,
      height,
      weight,
      age,
      bloodPressure,
      heartRate,
      temperature,
      alcoholUse,
      smoking,
      drugUse,
      activeSection
    };
    saveFormState(formState);
  }, [
    selectedSymptoms,
    selectedMedications,
    selectedAllergies,
    genderOther,
    raceOther,
    selectedGender,
    selectedRace,
    height,
    weight,
    age,
    bloodPressure,
    heartRate,
    temperature,
    alcoholUse,
    smoking,
    drugUse,
    activeSection
  ]);

  // File upload mutation
  const mutation = useMutation(
    (formData) => axios.post('https://backend-service-646481361829.us-central1.run.app/api/uploadFile', formData),
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
        if (data.bloodpressure) setBloodPressure(Number(data.bloodpressure));
        if (data.heartrate) setHeartRate(Number(data.heartrate));
        if (data.temperature) setTemperature(Number(data.temperature));
        if (data.medications) setSelectedMedications(data.medications.map(m => ({ value: m, label: m })));
        if (data.allergies) setSelectedAllergies(data.allergies.map(a => ({ value: a, label: a })));
        if (data.alcohol) setAlcoholUse(data.alcohol);
        if (data.smoking) setSmoking(data.smoking);
        if (data.druguse) setDrugUse(data.druguse);
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
        bloodpressure: bloodPressure ? parseInt(bloodPressure) : 0,
        heartrate: heartRate ? parseInt(heartRate) : 0,
        temperature: temperature ? parseFloat(temperature) : 0,
        medications: selectedMedications.map(m => m.value),
        allergies: selectedAllergies.map(a => a.value),
        alcohol: alcoholUse || "0",
        smoking: smoking || "0",
        druguse: drugUse || "0",
      };
      
      const response = await axios.post('https://backend-service-646481361829.us-central1.run.app/api/uploadForm', formData, {
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

  // Custom change handlers for toggleable radio buttons
  const handleGenderChange = (e) => {
    const value = e.target.value;
    setSelectedGender(prev => prev === value ? '' : value);
  };

  const handleRaceChange = (e) => {
    const value = e.target.value;
    setSelectedRace(prev => prev === value ? '' : value);
  };

  const handleAlcoholUseChange = (e) => {
    const value = e.target.value;
    setAlcoholUse(prev => prev === value ? '' : value);
  };

  const handleSmokingChange = (e) => {
    const value = e.target.value;
    setSmoking(prev => prev === value ? '' : value);
  };

  const handleDrugUseChange = (e) => {
    const value = e.target.value;
    setDrugUse(prev => prev === value ? '' : value);
  };

  // Custom RadioButton component that supports toggling
  const RadioButton = ({ name, id, value, checked, onChange, label }) => {
    return (
      <div className="form-check">
        <input
          className="form-check-input"
          type="radio"
          name={name}
          id={id}
          value={value}
          checked={checked}
          onChange={onChange}
          onClick={(e) => {
            if (checked) {
              e.preventDefault();
              onChange({ target: { value: '' } });
            }
          }}
        />
        <label className="form-check-label" htmlFor={id}>
          {label}
        </label>
      </div>
    );
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
                    className={`nav-link ${activeSection === 'symptoms' ? 'active' : ''}`}
                    onClick={() => setActiveSection('symptoms')}
                  >
                    Symptoms
                  </button>
                </li>
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
                        <RadioButton
                          name="gender"
                          id="genderMale"
                          value="Male"
                          checked={selectedGender === 'Male'}
                          onChange={handleGenderChange}
                          label="Male"
                        />
                        <RadioButton
                          name="gender"
                          id="genderFemale"
                          value="Female"
                          checked={selectedGender === 'Female'}
                          onChange={handleGenderChange}
                          label="Female"
                        />
                        <RadioButton
                          name="gender"
                          id="genderOther"
                          value="Other"
                          checked={selectedGender === 'Other'}
                          onChange={handleGenderChange}
                          label="Other"
                        />
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
                        <RadioButton
                          name="race"
                          id="raceWhite"
                          value="White"
                          checked={selectedRace === 'White'}
                          onChange={handleRaceChange}
                          label="White"
                        />
                        <RadioButton
                          name="race"
                          id="raceBlack"
                          value="Black"
                          checked={selectedRace === 'Black'}
                          onChange={handleRaceChange}
                          label="Black/African American"
                        />
                        <RadioButton
                          name="race"
                          id="raceAsian"
                          value="Asian"
                          checked={selectedRace === 'Asian'}
                          onChange={handleRaceChange}
                          label="Asian"
                        />
                        <RadioButton
                          name="race"
                          id="raceHispanic"
                          value="Hispanic"
                          checked={selectedRace === 'Hispanic'}
                          onChange={handleRaceChange}
                          label="Hispanic/Latino"
                        />
                        <RadioButton
                          name="race"
                          id="raceOther"
                          value="Other"
                          checked={selectedRace === 'Other'}
                          onChange={handleRaceChange}
                          label="Other"
                        />
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
                          value={bloodPressure}
                          onChange={(e) => setBloodPressure(e.target.value)}
                        />
                      </div>
                      <div className="col-md-4 mb-3">
                        <label htmlFor="heartrate" className="form-label">Heart Rate (bpm):</label>
                        <input 
                          type="number" 
                          id="heartrate" 
                          name="heartrate" 
                          className="form-control" 
                          value={heartRate}
                          onChange={(e) => setHeartRate(e.target.value)}
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
                          value={temperature}
                          onChange={(e) => setTemperature(e.target.value)}
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
                        <RadioButton
                          name="alcoholuse"
                          id="alcoholNever"
                          value="Never"
                          checked={alcoholUse === 'Never'}
                          onChange={handleAlcoholUseChange}
                          label="Never"
                        />
                        <RadioButton
                          name="alcoholuse"
                          id="alcoholRarely"
                          value="Rarely"
                          checked={alcoholUse === 'Rarely'}
                          onChange={handleAlcoholUseChange}
                          label="Rarely"
                        />
                        <RadioButton
                          name="alcoholuse"
                          id="alcoholMonthly"
                          value="Monthly"
                          checked={alcoholUse === 'Monthly'}
                          onChange={handleAlcoholUseChange}
                          label="Monthly"
                        />
                        <RadioButton
                          name="alcoholuse"
                          id="alcoholWeekly"
                          value="Weekly"
                          checked={alcoholUse === 'Weekly'}
                          onChange={handleAlcoholUseChange}
                          label="Weekly"
                        />
                        <RadioButton
                          name="alcoholuse"
                          id="alcoholDaily"
                          value="Daily"
                          checked={alcoholUse === 'Daily'}
                          onChange={handleAlcoholUseChange}
                          label="Daily"
                        />
                      </div>
                      
                      <div className="col-md-4 mb-3">
                        <label className="form-label">Smoking:</label>
                        <RadioButton
                          name="smoking"
                          id="smokingNever"
                          value="Never"
                          checked={smoking === 'Never'}
                          onChange={handleSmokingChange}
                          label="Never"
                        />
                        <RadioButton
                          name="smoking"
                          id="smokingRarely"
                          value="Rarely"
                          checked={smoking === 'Rarely'}
                          onChange={handleSmokingChange}
                          label="Rarely"
                        />
                        <RadioButton
                          name="smoking"
                          id="smokingMonthly"
                          value="Monthly"
                          checked={smoking === 'Monthly'}
                          onChange={handleSmokingChange}
                          label="Monthly"
                        />
                        <RadioButton
                          name="smoking"
                          id="smokingWeekly"
                          value="Weekly"
                          checked={smoking === 'Weekly'}
                          onChange={handleSmokingChange}
                          label="Weekly"
                        />
                        <RadioButton
                          name="smoking"
                          id="smokingDaily"
                          value="Daily"
                          checked={smoking === 'Daily'}
                          onChange={handleSmokingChange}
                          label="Daily"
                        />
                      </div>
                      
                      <div className="col-md-4 mb-3">
                        <label className="form-label">Drug Use:</label>
                        <RadioButton
                          name="druguse"
                          id="druguseNever"
                          value="Never"
                          checked={drugUse === 'Never'}
                          onChange={handleDrugUseChange}
                          label="Never"
                        />
                        <RadioButton
                          name="druguse"
                          id="druguseRarely"
                          value="Rarely"
                          checked={drugUse === 'Rarely'}
                          onChange={handleDrugUseChange}
                          label="Rarely"
                        />
                        <RadioButton
                          name="druguse"
                          id="druguseMonthly"
                          value="Monthly"
                          checked={drugUse === 'Monthly'}
                          onChange={handleDrugUseChange}
                          label="Monthly"
                        />
                        <RadioButton
                          name="druguse"
                          id="druguseWeekly"
                          value="Weekly"
                          checked={drugUse === 'Weekly'}
                          onChange={handleDrugUseChange}
                          label="Weekly"
                        />
                        <RadioButton
                          name="druguse"
                          id="druguseDaily"
                          value="Daily"
                          checked={drugUse === 'Daily'}
                          onChange={handleDrugUseChange}
                          label="Daily"
                        />
                      </div>
                    </div>
                  </div>
                </div>
              </form>

              <div className="d-flex justify-content-between mt-4">
                <button
                  type="submit" 
                  form="medicalForm" 
                  className="btn btn-success"
                  onClick={() => {
                    if (selectedSymptoms.length === 0) {
                      alert('Please enter at least one symptom before submitting.');
                      setActiveSection('symptoms');
                      return;
                    }
                  }}
                >
                  Submit Form
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}