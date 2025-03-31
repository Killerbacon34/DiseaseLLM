import React from 'react';

let id = 0;
function createData(patientName, diagnosis, treatment, drug) {
  id += 1;
  return { id, patientName, diagnosis, treatment, drug};
}

let rows = [
  createData('John Doe', 'Cancer', 'Chemotherapy', 'Tylenol'),
  //make call to database to get the data using: createData('Patient Name', 'Diagnosis', 'Treatment Plan', 'Drug Usage Plan'),
];

  export default class Approval extends React.Component {
  render() {
    return(
      <>
      <div style ={{fontWeight: 'bold', justifyContent: 'center', alignItems: 'center'}}>
        <h1>Generated Diagnoses</h1> 
      </div>
      <table class="table table-hover">
        <thead>
          <tr onClick={this.handleRowClick}>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'white', height: 70}}>Patient Name</th>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'white', height: 70}}>Diagnosis</th>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'white', height: 70}}>Treatment Plan</th>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'white', height: 70}}>Drug Usage Plan</th>
          </tr>
        </thead>
        <tbody>
            {rows.map(row => (
            <tr key={row.id} class="table-active">
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'white'}}>{row.patientName}</td>
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'white'}}>{row.diagnosis}</td>
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'white'}}>{row.treatment}</td>
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'white'}}>{row.drug}</td>
            </tr>
         ))}
        </tbody>
      </table>
      </>
    )
  }

}

// export default withRouter(Approval);