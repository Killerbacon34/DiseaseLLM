export default function Comment() {
    return (<div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', height: '100vh', paddingTop: '1rem' }}>
      <h1>Comments</h1>
      <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', marginTop: '1rem' }}>
      <form>
          <label htmlfor="patientName">Patient Name:</label><br></br>
          <input type="text" id="patientName" name="patientName"></input><br></br>
          <label htmlfor="diagnosis">Diagnosis:</label><br></br>
          <input type="text" id="diagnosis" name="diagnosis"></input><br></br>
          <label htmlfor="treatmentPlan">Treatment Plan:</label><br></br>
          <input type="text" id="treatmentPlan" name="treatmentPlan"></input><br></br>
          <label htmlfor="drugUsagePlan">Drug Usage Plan:</label><br></br>
          <input type="text" id="drugUsagePlan" name="drugUsagePlan"></input><br></br>
          <input type="submit" value="Approve"></input>
          <input type="submit" value="Reject"></input>
      </form>
      </div>
      
    </div>)
  }