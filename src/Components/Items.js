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
const TOKEN_ID = "008";

const GAS = "200000000000000";

  const Items =(props)=> {
		// mint button behavoiur
		const mintButton = async () => {

			await window.contract.mint_now({
			token_id: TOKEN_ID,
			receiver_id: window.accountId,
			token_metadata: props.metadata
			}, GAS, deposit);
		}

      return (
              <div>
              <Card style={{ width: '27rem' }} className="text-center" border="dark">
                  <Card.Header>{props.metadata.title}</Card.Header>
                  <Card.Body>

                    <Card.Img src={props.picture}/>
                    <Card.Text>{props.metadata.description}</Card.Text>
                    <Container>
                    <Row className="rowSpacing d-flex justify-content-center">
                      <Col><Card.Title>2.9 NEAR</Card.Title></Col>
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
