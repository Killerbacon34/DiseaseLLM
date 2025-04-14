import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useRouter, useSearchParams } from 'next/navigation';
import { Button, Container, Row, Col } from 'react-bootstrap';

export default function Release() {
    const router = useRouter();
    const searchParams = useSearchParams(); // Access query parameters
    const previousPage = searchParams.get('from'); // Get the 'from' query parameter

    const handleClick = async (e) => {
        e.preventDefault();
        try {
            let apiUrl = 'http://localhost:4545/anon/release'; // Default API
            if (previousPage === '/signup') {
                apiUrl = 'http://localhost:4545/auth/release'; // API for signup page
            }

            const response = await axios.get(apiUrl, {
                withCredentials: true,
            }
            );
            if (response.status !== 200) {
                throw new Error('Failed to fetch data');
            }
            //sessionStorage.setItem('Auth', response.data.token);
            //sessionStorage.setItem('anonid', response.data.anonid);
            router.push('/upload');
        } catch (err) {
            console.log(err);
        }
    };

//     return (
//         <div>
//             <h1>Medical Information Release Form</h1>
//             <div>
//                 <p style={{textweight: 'bold'}}>I understand the following:</p>
//                 <p> 1. I authorize the use or disclosure of the health information as described above for the purpose listed. I understand this authorization is voluntary.</p>
//                 <p> 2. I have the right to revoke this authorization. To do so I understand I must
//                     submit my revocation in writing to the party entered in Part II. The revocation
//                     will prevent further release of my health information from the date of receipt.</p>
//                 <p> 3. I am signing this authorization voluntarily and understand my health care
//                     treatment will not be affected if I do not sign this authorization.</p>
//                 <p >4. The party entered in Part III is prohibited from re-disclosing the health
//                     information except with a written authorization or as specifically permitted by
//                     law.</p>
//                 <p> 5. If the party entered in Part III is not a HIPAA Covered Entity or Business
//                     Associate as defined in 45 CFR §160.103, the released health information may no
//                     longer be protected by federal and state privacy regulations.</p>
//                 <p> 6. I have a right to receive a copy of this authorization.</p>
//                 <p> 7. Fees may be charged to cover the cost of releasing the health information.</p>
//                 <p> 8. I understand that my substance abuse disorder records are protected under
//                     the federal regulations governing the Confidentiality of Substance Use Disorder
//                     Patient Records and cannot be redisclosed without my written authorization.</p>
//             </div>
//             <button type="button" onClick={handleClick}>ACCEPT</button>
//         </div>
//     );
// }
return (
    <Container className="my-5">
  <Row>
    <Col md={{ span: 8, offset: 2 }}>
      <h1 className="text-center mb-4 ">Medical Information Release Form</h1>
      <div className="bg-light p-4 rounded shadow-sm">
        <h3 className="fw-bolder text-center">I understand the following:</h3>
        <p></p>
        <ul className="ps-36">
          <li>
            <p>
            I authorize the use or disclosure of the health information as described above for the purpose listed. I understand this authorization is voluntary.
            </p>
          </li>
          <li>
            <p>
            I have the right to revoke this authorization. To do so I must submit my revocation to the party responsible for this yet. The revocation will prevent further release of my health information from the date of receipt.
            </p>
          </li>
          <li>
            <p>
            I am accepting this authorization voluntarily and understand my health care treatment will not be affected if I do not sign this authorization.
            </p>
          </li>
          <li>
            <p>
            The party responsible for this product is prohibited from re-disclosing the health information except with a written authorization or as specifically permitted by law.
            </p>
          </li>
          <li>
            <p>
            If the party responsible for this product is not a HIPAA Covered Entity or Business Associate, the released health information may no longer be protected by federal and state privacy regulations.
            </p>
          </li>
          <li>
            <p>
            I have a right to receive a copy of this authorization.
            </p>
          </li>
          <li>
            <p>
            Fees may be charged to cover the cost of releasing the health information.
            </p>
          </li>
          <li>
            <p>
            I understand that my substance abuse disorder records are protected under the federal regulations governing the Confidentiality of Substance Use Disorder Patient Records and cannot be redisclosed without my written authorization.
            </p>
          </li>
        </ul>
        <div className="text-center mt-4">
          <Button variant="primary" onClick={handleClick}>
            ACCEPT
          </Button>
        </div>
      </div>
    </Col>
  </Row>
</Container>
);
}