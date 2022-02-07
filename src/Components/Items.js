import React from 'react';
import {Card, Button, Row, Col, Container} from 'react-bootstrap'
import * as nearAPI from 'near-api-js';
export const {
	utils: {
		format: {
			formatNearAmount, parseNearAmount
		}
	}
} = nearAPI;


const deposit = parseNearAmount('0.1');
const TOKEN_ID = "001";
var TOKEN_METADATA = { 'title': 'Olympus Mons', 'description': 'Tallest mountain in charted solar system', 'media': 'https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg', 'copies': 1};

const GAS = "200000000000000";

  const Items =(props)=> {
		// mint button behavoiur
		const mintButton = async () => {

			await window.contract.mint_now({
			token_id: TOKEN_ID,
			receiver_id: window.accountId,
			token_metadata: TOKEN_METADATA
			}, GAS, deposit);
		}

      return (
              <div>
              <Card style={{ width: '20rem' }} className="text-center" border="dark">
                  <Card.Header>{props.name}</Card.Header>
                  <Card.Body>

                    <Card.Img src={props.picture}/>
                    <Card.Text>{props.description}</Card.Text>
                    <Container>
                    <Row className="rowSpacing d-flex justify-content-center">
                      <Col><Card.Title>{props.price}</Card.Title></Col>
                      </Row>
                    <Row className="rowSpacing d-flex justify-content-center">
                      <Col><Button onClick={mintButton}>Mint</Button></Col>
                      <Col><Button>Buy now</Button></Col>
                      </Row>
                    </Container>

                  </Card.Body>
                </Card>
              </div>
            );

  }

  export default Items;
