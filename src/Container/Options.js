import React, {Component} from 'react';
import {Container, Row, Card} from 'react-bootstrap';

import Item from './../Components/Items.js'
import './../css/reactAppUtils.css'


  class Options extends Component {


    listItems = [
      {metadata: { 'title': 'Olympus Mons', 'description': 'Tallest mountain in charted solar system',
    'media': 'https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg',
    'copies': 1}}
    ]


    render() {
      return (
              <div className="OptionsContainer">
                <Container>
                {
                  this.listItems.map(x=>{
                    return <Row>
                      <Item metadata={x.metadata} picture={x.metadata.media}/>

                    </Row>
                  })
                }
                </Container>

              </div>
            );
    }
  }

  export default Options;
