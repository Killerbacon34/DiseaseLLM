export default function ManualUpload() {
    return (
    <div className="d-flex justify-content-center vh-100">
      <div>
      <h1 className="text-center mb-3">Upload</h1>
      <div className="d-flex align-items-center justify-content-center">
      <form>
          <label htmlfor="patientName" className="form-label">Patient Name:</label><br></br>
          <input type="text" id="patientName" name="patientName" className="form-control"></input><br></br>
          <label htmlfor="Symptoms" className="form-label">Symptoms:</label><br></br>
          <input type="text" id="Symptoms" name="Symptoms" className="form-control"></input><br></br>
          <label htmlfor="treatmentPlan" className="form-label">Treatment Plan:</label><br></br>
          <input type="text" id="treatmentPlan" name="treatmentPlan" className="form-control"></input><br></br>
          <label htmlfor="drugUsagePlan" className="form-label">Drug Usage Plan:</label><br></br>
          <input type="text" id="drugUsagePlan" name="drugUsagePlan" className="form-control"></input><br></br>
          <input type="submit" value="Submit" className="btn btn-primary w-100"></input>
      </form>
      </div>
      </div>
      
    </div>)
  }