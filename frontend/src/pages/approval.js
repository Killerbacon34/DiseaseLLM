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

// function withRouter(Component) {
//   return function (props) {
//     const navigate = useNavigate();
//     return <Component {...props} navigate={navigate} />;
//   };
// }

// class Approval extends React.Component {
  // handleRowClick = () => {
  //   this.props.navigate("/comment");
  // };
  export default class Approval extends React.Component {
  render() {
    return(
      <>
      <div style ={{fontWeight: 'bold', justifyContent: 'center', alignItems: 'center'}}>
        <h1>Approval</h1> 
      </div>
      <table style={{width: '100%', border: 1, borderStyle: 'solid', borderColor: 'black'}}>
        <thead>
          <tr onClick={this.handleRowClick}>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'black', height: 70}}>Patient Name</th>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'black', height: 70}}>Diagnosis</th>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'black', height: 70}}>Treatment Plan</th>
            <th style={{fontWeight: 'bold', border: 1, borderStyle: 'solid', borderColor: 'black', height: 70}}>Drug Usage Plan</th>
          </tr>
        </thead>
        <tbody>
            {rows.map(row => (
            <tr key={row.id}>
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'black'}}>{row.patientName}</td>
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'black'}}>{row.diagnosis}</td>
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'black'}}>{row.treatment}</td>
                <td style={{border: 1, borderStyle: 'solid', borderColor: 'black'}}>{row.drug}</td>
            </tr>
         ))}
        </tbody>
      </table>
      </>
    )
  }

}

// export default withRouter(Approval);