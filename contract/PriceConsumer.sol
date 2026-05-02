// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AggregatorV3Interface} from "@chainlink/contracts/src/v0.8/shared/interfaces/AggregatorV3Interface.sol";

// Simple program to get the price ETH/USD
contract PriceConsumer{
    // Variable dataFeed
    AggregatorV3Interface internal dataFeed;

    // Define constructor with the address of the chainlink price feed
    constructor(){
        dataFeed = AggregatorV3Interface(
            0x694AA1769357215DE4FAC081bf1f309aDC325306
        );
    }

    // Public view function to return the price
    function getLatestPrice() public view returns(int){
        (,int price,,,) = dataFeed.latestRoundData();
        return price;
    }
}
