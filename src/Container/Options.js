import React, {Component} from 'react';
import {Container, Row, Card} from 'react-bootstrap';

import Item from './../Components/Items.js'
import './../css/reactAppUtils.css'


  class Options extends Component {


    listItems = [
      {metadata: { 'title': 'Near', 'description': "Near (ニア, Nia) is the younger of L's two successors, raised in Wammy's House—Watari's orphanage for gifted children in Winchester, England",
    'media': 'https://bafybeia5tdrinbi7m2dhnpsahizkagn5cshrmqyhjnwig4vtfgdsnhcdkm.ipfs.dweb.link',
    'copies': 1, 'token_id': '1200'}}
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
