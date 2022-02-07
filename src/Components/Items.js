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


const DEPOSIT = parseNearAmount('0.1');
const TOKEN_ID = "099";
const GAS = "200000000000000";
let price

  const Items =(props)=> {
		fetchPrice();
		// mint button behavoiur
		const mintButton = async () => {
			await window.contract.mint_now({
			token_id: TOKEN_ID,
			receiver_id: window.accountId,
			token_metadata: props.metadata
			}, GAS, DEPOSIT);
		}
		// buy button behavoiur
		const buyNowButton = async () => {
			await window.contract.near_transfer({
			to: "devdoor5.testnet",
			amount: price
		}, GAS, price);
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
                      <Col><Button onClick={buyNowButton}>Buy now</Button></Col>
                      </Row>
                    </Container>

                  </Card.Body>
                </Card>
              </div>
            );

  }

	async function fetchPrice() {
		price = await contract.get_price()
	}

  export default Items;
