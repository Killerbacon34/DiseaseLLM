export default function Comment() {
    return (
    <div className="d-flex justify-content-center vh-100">
      <div>
      <h1 className="text-center mb-3">Comments</h1>
      <div className="d-flex align-items-center justify-content-center">
      <form>
          <label htmlfor="patientName" className="form-label">Patient Name:</label><br></br>
          <input type="text" id="patientName" name="patientName" className="form-control"></input><br></br>
          <label htmlfor="diagnosis" className="form-label">Diagnosis:</label><br></br>
          <input type="text" id="diagnosis" name="diagnosis" className="form-control"></input><br></br>
          <label htmlfor="treatmentPlan" className="form-label">Treatment Plan:</label><br></br>
          <input type="text" id="treatmentPlan" name="treatmentPlan" className="form-control"></input><br></br>
          <label htmlfor="drugUsagePlan" className="form-label">Drug Usage Plan:</label><br></br>
          <input type="text" id="drugUsagePlan" name="drugUsagePlan" className="form-control"></input><br></br>
          <input type="submit" value="Approve" className="btn btn-success w-100"></input>
          <input type="submit" value="Reject" className="btn btn-danger w-100"></input>
      </form>
      </div>
      </div>
      
    </div>)
  }