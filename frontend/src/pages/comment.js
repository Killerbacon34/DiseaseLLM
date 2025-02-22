export default function Comment() {
    return (<div style={{
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      height: '100vh'
      }}>
      <h1>Comments</h1>
      <div>
      <form>
          <label for="patientName">Patient Name:</label><br></br>
          <input type="text" id="patientName" name="patientName"></input><br></br>
          <label for="diagnosis">Diagnosis:</label><br></br>
          <input type="text" id="diagnosis" name="diagnosis"></input><br></br>
          <label for="treatmentPlan">Treatment Plan:</label><br></br>
          <input type="text" id="treatmentPlan" name="treatmentPlan"></input><br></br>
          <label for="drugUsagePlan">Drug Usage Plan:</label><br></br>
          <input type="text" id="drugUsagePlan" name="drugUsagePlan"></input><br></br>
          <input type="submit" value="Submit"></input>
      </form>
      </div>
      
    </div>)
  }