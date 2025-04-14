// import Link from "next/link";


// const Navbar = () => {
//   return (
//     <nav style={{backgroundcolor: 'blue',
//     display: 'flex',
//     justifyContent: 'space-between',
//     padding: '1rem',
//     alignItems: 'center',
//     color: 'white'
//     }}>
//       <div style={{display: 'flex', alignItems: 'center', fontWeight: 'bold', fontSize: '1.5rem'}}>
//         <Link href="/">Home</Link>
//       </div>
//       <div style={{display: 'flex', marginRight: '1rem', fontWeight: 'bold', fontSize: '1.5rem'}}>
//         <Link href="/signup">Sign Up</Link>
//       </div>
//       <div style={{display: 'flex', marginRight: '1rem', fontWeight: 'bold', fontSize: '1.5rem'}}>
//         <Link href="/upload">Upload</Link>
//       </div>
//     </nav>
//   );
// };

import React, { useState, useEffect } from 'react';
import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import Offcanvas from 'react-bootstrap/Offcanvas';
import { BsJustify, BsX } from "react-icons/bs";

function MyNavbar() {
  const [show, setShow] = useState(false);
  const [user, setUser] = useState(false);

  useEffect(() => {
    const token = sessionStorage.getItem('Auth');
    if (token) {
      // Simulate a user check based on the token
      setUser({ username: 'User' }); // Replace with actual user fetching logic
    }
  }, []);  

  const handleShow = () => setShow(true);
  const handleClose = () => setShow(false);

  return (
    <>
    <div>
    {!show && (
      <Button 
        variant="primary" 
        onClick={handleShow} 
        style={{ position: "fixed", top: "20px", right: "20px", zIndex: 1050 }}
      >
        <BsJustify size={24} />
      </Button>
    )}

      <Offcanvas show={show} onHide={handleClose} placement="end">
        <Offcanvas.Header>
        <Button variant="link" onClick={handleClose} style={{ marginRight: "auto" }}>
            <BsX size={24} />
          </Button>
          <Offcanvas.Title>Options</Offcanvas.Title>
        </Offcanvas.Header>
        <Offcanvas.Body>
          <Nav>
              <Nav.Link href="/">Home</Nav.Link>
              {!user && <Nav.Link href="/signup">Sign Up</Nav.Link>}
              {!user && <Nav.Link href="/login">Login</Nav.Link>}
              {/* <Nav.Link href="/upload">Document Upload</Nav.Link> */}
              <Nav.Link href="/release">Enter Information</Nav.Link>
              <Nav.Link href="/result">View Diagnosis</Nav.Link>
          </Nav>
        </Offcanvas.Body>
      </Offcanvas>
      </div>
    </>
  );
}

export default MyNavbar;
