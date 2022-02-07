import React from 'react';
import {Card, Button, Row, Col, Container} from 'react-bootstrap'

const gas_price = "200000000000000";
const mint_deposit = "100000000000000000000000"; // 0.1 NEAR
const price = "2900000000000000000000000";

  const Items =(props)=> {

		// mint button behavoiur
		const mintButton = async () => {
      console.log(gas_price)
			await window.contract.mint_now({
			token_id: props.token_id,
			receiver_id: window.accountId,
			token_metadata: props.metadata
    }, gas_price, mint_deposit);
		}
		// buy button behavoiur
		const buyNowButton = async () => {
			await window.contract.near_transfer({
			to: window.accountId,
			amount: price
		}, gas_price, price);
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

  /*async function fetchPrice() {
    price = await window.contract.get_price()
  }
  async function fetchGasPrice() {
    gas_price = await window.contract.get_gas_price()
  }
  async function fetchDepositPrice() {
    mint_deposit = await window.contract.get_mint_deposit()
  }*/


  export default Items;
