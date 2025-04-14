import { Container, Row, Col, Card, Button } from "react-bootstrap";

export default function Home() {
  return (
    <div className="homepage bg-light">
      <header className="hero-section py-5 bg-primary text-white">
        <Container>
          <Row className="align-items-center">
            <Col lg={6}>
              <h1 className="display-4 fw-bold mb-4">Disease LLM Medical Diagnostics</h1>
              <p className="lead mb-4">
                Harnessing the power of multiple large language models to provide concise diagnostic insights for everyday use.
              </p>
              <a href="/release" className="btn btn-light btn-lg me-3">
                Get a Diagnosis Here
              </a>
            </Col>
            <Col lg={6} className="d-none d-lg-block">
              <div style={{ 
                backgroundColor: 'transparent',
                display: 'flex',           
                justifyContent: 'flex-end', 
                padding: '20px',
                marginRight: '100px'         
              }}>
                <img 
                  src="Logo.png"
                  alt="Logo"
                  className="img-fluid"
                  style={{ 
                    maxHeight: '350px',
                    background: 'transparent',
                    filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.1))'
                    
                  }}
                />
              </div>
            </Col>
          </Row>
        </Container>
      </header>

      <section className="py-5">
        <Container>
          <h2 className="text-center mb-5 fw-bold">Our Diagnostic Capabilities</h2>
          <Row>
            <Col md={4} className="mb-4">
              <Card className="h-100 shadow-sm">
                <Card.Body className="text-center">
                  <div className="icon-box bg-soft-primary mb-3">
                    <i className="fas fa-brain fa-3x text-primary"></i>
                  </div>
                  <Card.Title>Multi-Model Analysis</Card.Title>
                  <Card.Text>
                    Synthesizes insights from multiple LLMs to provide comprehensive diagnostic suggestions.
                  </Card.Text>
                </Card.Body>
              </Card>
            </Col>
            <Col md={4} className="mb-4">
              <Card className="h-100 shadow-sm">
                <Card.Body className="text-center">
                  <div className="icon-box bg-soft-success mb-3">
                    <i className="fas fa-chart-line fa-3x text-success"></i>
                  </div>
                  <Card.Title>User Friendly</Card.Title>
                  <Card.Text>
                    Intuitive interface designed for ease of use, allowing users to receive medical assistance quickly.
                  </Card.Text>
                </Card.Body>
              </Card>
            </Col>
            <Col md={4} className="mb-4">
              <Card className="h-100 shadow-sm">
                <Card.Body className="text-center">
                  <div className="icon-box bg-soft-info mb-3">
                    <i className="fas fa-shield-alt fa-3x text-info"></i>
                  </div>
                  <Card.Title>HIPAA Compliant</Card.Title>
                  <Card.Text>
                    Secure platform designed with patient privacy and data protection as top priorities.
                  </Card.Text>
                </Card.Body>
              </Card>
            </Col>
          </Row>
        </Container>
      </section>

      <section className="py-5 bg-dark text-white">
        <Container className="text-center">
          <h2 className="mb-4">Ready to enhance your medical aid?</h2>
          <p className="lead mb-4">Create an account here and be able to see your previous diagnoses</p>
          <a href="/signup" className="btn btn-light btn-lg px-4">
            Sign Up Now
          </a>
        </Container>
      </section>
    </div>
  );
}