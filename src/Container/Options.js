import React, {Component} from 'react';
import {Container, Row, Card} from 'react-bootstrap';
//import sara from './../assets/sara.jpg'
const sara = 'https://www.publicdomainpictures.net/pictures/200000/velka/lady-on-vintage-postcard-1474314845SA7.jpg'
import Item from './../Components/Items.js'
import './../css/reactAppUtils.css'


  class Options extends Component {

    listItems = [
      {name:"sara.jpg", price:30, description:"befana", picture:sara}
    ]


    render() {
      return (
              <div className="OptionsContainer">
                <Container>
                {
                  this.listItems.map(x=>{
                    return <Row>
                      <Item name={x.name} price={x.price} description={x.description} picture={x.picture}/>

                    </Row>
                  })
                }
                </Container>

              </div>
            );
    }
  }

  export default Options;
