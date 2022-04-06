<template>
<div class="middle">


<el-input v-model="count" placeholder="请数量"></el-input>
<div style="margin-bottom:30px;margin-top:30px;"> <el-button type="primary" @click="transfer()">ERC20 to mainbridge on Rinkeby</el-button></div>
<div> <el-button type="primary" @click="returntorinkby()">ERC20 to sidebridge on Ropsten</el-button></div>


</div>
</template>


<script>

import Web3 from 'web3';

import detectEthereumProvider from '@metamask/detect-provider';

 var my_account;


function getDataFieldValue(tokenRecipientAddress, tokenAmount) {
    const web3 = new Web3();
    const TRANSFER_FUNCTION_ABI = {"constant":false,"inputs":[{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transfer","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"};
    return web3.eth.abi.encodeFunctionCall(TRANSFER_FUNCTION_ABI, [
        tokenRecipientAddress,
        tokenAmount
    ]);
}


  export default {
    data() {
      return {
       count:0.000000000001,
       data:''
      };
    },
    methods: {

async transfer() {

const provider = await detectEthereumProvider();

if (provider) {
  // From now on, this should always be true:
  // provider === window.ethereum
   window.web3=new Web3(window.ethereum);
  console.log("sfsdfsfsfsdf");
} else {
  	window.alert('Please install MetaMask first.');
        //  https://metamask.io/download/
		    window.open("https://metamask.io/download/","_blank");
			return;
}
 

		if (window.ethereum) {
           
			  console.log("1-------------------------");
			try {
				 const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
        // window.web3=new web3(window.ethereum);

	        my_account=accounts;
			} catch (error) {
				window.alert('You need to allow MetaMask.');
				return;
			}

			  console.log("2------------------------");


		}
 
		//const coinbase = await web3.eth.getCoinbase();
		if (!my_account) {
			window.alert('Please activate MetaMask first.');
			return;
		}

      console.log("the adress is ",my_account);

  const transactionParameters = {
    from: my_account[0],
    to: "0x2D087e51eD54a348de214E0B39d85Be5976D7779",
    data: getDataFieldValue("0x923C2d576eEb35644447d177940088Fa2a94b5d6", this.count*Math.pow(10,18)),
};
let res=await ethereum.request({
    method: 'eth_sendTransaction',
    params: [transactionParameters],
});

	  console.log("3------------------------",res);
    
      },

     async returntorinkby() {

const provider = await detectEthereumProvider();

if (provider) {
  // From now on, this should always be true:
  // provider === window.ethereum
   window.web3=new Web3(window.ethereum);

} else {
  	window.alert('Please install MetaMask first.');
		    window.open("https://metamask.io/download/","_blank");
			return;
}
 

		if (window.ethereum) {
           
			  console.log("1-------------------------");
			try {
				 const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
        // window.web3=new web3(window.ethereum);

	        my_account=accounts;
			} catch (error) {
				window.alert('You need to allow MetaMask.');
				return;
			}

			  console.log("2------------------------");


		}
 
		//const coinbase = await web3.eth.getCoinbase();
		if (!my_account) {
			window.alert('Please activate MetaMask first.');
			return;
		}

      console.log("the adress is ",my_account);

  const transactionParameters = {
    from: my_account[0],
    to: "0xE0daEd63ce045833C22862A7fA3a95527DF8bcdC",
    data: getDataFieldValue("0x5e4A4859d1Af4A315080DEb43B9bFB01Fa2016ef", this.count*Math.pow(10,18)),
};
let res=await ethereum.request({
    method: 'eth_sendTransaction',
    params: [transactionParameters],
});

	  console.log("3------------------------",res);

    }

    
    },
    beforeCreate() {
      
    },
    mounted() {
    

    },

    beforeUpdate() {
    
    },
    updated() {
  
    },
    destroyed() {
   
    }
  }
</script>

<style>
  .text {
    font-size: 14px;
  }

</style>

