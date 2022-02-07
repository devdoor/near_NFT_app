import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'
import './scss/AppStyles.scss';
import Options from './Container/Options.js'

// Bootstrap Components
import "bootstrap/dist/css/bootstrap.min.css";
import { Container, Row, Col, Nav, Navbar, Card, NavDropdown} from "react-bootstrap";

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {

    return (
      <div className="App">
      <Navbar bg="dark" variant="dark" expand="lg">
        <Container>
          <Navbar.Brand>Mint NFT</Navbar.Brand>
          <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="me-auto">
              </Nav>
              <Nav>
                {!window.walletConnection.isSignedIn() ? (
              <Nav.Link  onClick={login}>Login</Nav.Link>
            ):(
              <Nav.Link  onClick={logout}>Logout</Nav.Link>
            )}
            </Nav>
          </Navbar.Collapse>
        </Container>
        </Navbar>
        <main>

          <Container>
            <div class="col d-flex justify-content-center">
              <Row>
                  <Options/>
              </Row>
                </div>
          </Container>

        </main>

      </div>
    )
  }
