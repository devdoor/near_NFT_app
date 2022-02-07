import React, {Component} from 'react';
import {Container, Row, Card} from 'react-bootstrap';

import Item from './../Components/Items.js'
import './../css/reactAppUtils.css'


  class Options extends Component {

    listItems = [
      {metadata: { 'title': 'Near', 'description': "Near (ニア, Nia) is the younger of L's two successors, raised in Wammy's House—Watari's orphanage for gifted children in Winchester, England",
    'media': 'https://bafybeia5tdrinbi7m2dhnpsahizkagn5cshrmqyhjnwig4vtfgdsnhcdkm.ipfs.dweb.link',
    'copies': 1}, token_id: generateId(30)}
    ]


    render() {
      return (
              <div className="OptionsContainer">
                <Container>
                {
                  this.listItems.map(x=>{
                    return <Row>
                      <Item token_id={x.token_id} metadata={x.metadata} picture={x.metadata.media}/>

                    </Row>
                  })
                }
                </Container>

              </div>
            );
    }
  }

  // dec2hex :: Integer -> String
  // i.e. 0-255 -> '00'-'ff'
  function dec2hex (dec) {
    return dec.toString(16).padStart(2, "0")
  }

  // generateId :: Integer -> String
  function generateId (len) {
    var arr = new Uint8Array((len || 40) / 2)
    window.crypto.getRandomValues(arr)
    return Array.from(arr, dec2hex).join('')
  }
  export default Options;
