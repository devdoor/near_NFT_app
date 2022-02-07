near_NFT_app
==================

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/projects/devdoor/near_NFT_app>)

This work-in-progress application was created on February 2022 as a reply to the 'Chikai coding test' for the position of Smart Contracts engineer.
Many code snippets and components were borrowed from the following repositories available on GitHub:
  - near-examples/nft-market
  - near-examples/NFT
  - near-examples/DeCash-Rust
  - near-apps/payments-api
  - near/create-near-app

The app consists of a simple interface displaying one image and three buttons interacting with the NEAR testnet:

Login / Logout
-------------------------------------
Allows the user to connect / disconnect their wallet. Automatically toggled based on the log status of the user.

Mint
-------------------------------------
Allows a logged user to mint the displayed image as an NFT into their testnet wallet. The NFT ID is randomly generated.

Buy Now
-------------------------------------
Ideally should allow the user to pay a fixed amount to the previous owner of the NFT and transfer it to their wallet. Currently only the payment was implemented, with a fixed amount (2.9N) being transferred to the same wallet that called the method.

Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `yarn start`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

Step 1: Deploy the contract
------------------------------------------
One command:

    yarn start
