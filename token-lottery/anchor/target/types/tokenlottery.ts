/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/tokenlottery.json`.
 */
export type Tokenlottery = {
  "address": "4VNuGptng1b6vwb8fAMsy5Y5Kg7rKsaZT6bTXL9rrK8U",
  "metadata": {
    "name": "tokenlottery",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "intializeConfig",
      "discriminator": [
        38,
        75,
        134,
        154,
        249,
        64,
        246,
        46
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenLottery",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  108,
                  111,
                  116,
                  116,
                  101,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "startTime",
          "type": "u64"
        },
        {
          "name": "endTime",
          "type": "u64"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "tokenLottery",
      "discriminator": [
        219,
        174,
        104,
        58,
        76,
        30,
        61,
        218
      ]
    }
  ],
  "types": [
    {
      "name": "tokenLottery",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "winner",
            "type": "u64"
          },
          {
            "name": "winnerChosen",
            "type": "bool"
          },
          {
            "name": "startTime",
            "type": "u64"
          },
          {
            "name": "endTime",
            "type": "u64"
          },
          {
            "name": "lotteryPotAmount",
            "type": "u64"
          },
          {
            "name": "totalTickets",
            "type": "u64"
          },
          {
            "name": "ticketPrice",
            "type": "u64"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "randomnessAccount",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
