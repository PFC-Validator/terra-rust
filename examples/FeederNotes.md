# Oracle notes
GET /oracle/parameters

````json
{
  "height": "3565904",
  "result": {
    "vote_period": "5",
    "vote_threshold": "0.500000000000000000",
    "reward_band": "0.020000000000000000",
    "reward_distribution_window": "5256000",
    "whitelist": [
      {
        "name": "ukrw",
        "tobin_tax": "0.010000000000000000"
      },
      {
        "name": "usdr",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "uusd",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "umnt",
        "tobin_tax": "0.020000000000000000"
      },
      {
        "name": "ueur",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "ucny",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "ujpy",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "ugbp",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "uinr",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "ucad",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "uchf",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "uhkd",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "usgd",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "uaud",
        "tobin_tax": "0.003500000000000000"
      },
      {
        "name": "uthb",
        "tobin_tax": "0.007500000000000000"
      }
    ],
    "slash_fraction": "0.000100000000000000",
    "slash_window": "100800",
    "min_valid_per_window": "0.050000000000000000"
  }
}
````

GET /blocks/latest

```json
{
  "block_id": {
    "hash": "9B5D86CBD1BC4D85031F1C4C91AC670AC47E2A455F12A29982900C5F92D40D5B",
    "parts": {
      "total": "1",
      "hash": "B1C049DBC5081F001E80FA42960DE9B5A1141E3AF3E595874B4C76134799C4D9"
    }
  },
  "block": {
    "header": {
      "version": {
        "block": "10",
        "app": "0"
      },
      "chain_id": "tequila-0004",
      "height": "3565904",
      "time": "2021-04-20T02:38:25.783672445Z",
      "last_block_id": {
        "hash": "7E62C40A1F5623534FFA698BB2A1652A30FE677A8201AC8A9D2197FE4B400188",
        "parts": {
          "total": "1",
          "hash": "CEB66523C6265F2377059C498F3F518FF22832B634CBFE4652BC040F9E1DDCBE"
        }
      },
      "last_commit_hash": "841D8D2068F025B32648B3404756A4A271A4DE4471DF81240CB65724B72129DD",
      "data_hash": "D9EDACF8DE2560545DAAA8884F3AF74A8A861D97DF91C944B7C7E4B0030C36AA",
      "validators_hash": "D599F6B2D0EF7143AD372C5547D852CBB4DBE68A80DE066BC9F0C22306477EA9",
      "next_validators_hash": "D599F6B2D0EF7143AD372C5547D852CBB4DBE68A80DE066BC9F0C22306477EA9",
      "consensus_hash": "02A758797B629C1D78F134E76AF55F0D919057590451E74504C6CC82E7F076DF",
      "app_hash": "8CCA7BA7DB01328A95D48EAECDD0F9E0989132C0211152D3607233C333BC9154",
      "last_results_hash": "FE43D66AFA4A9A5C4F9C9DA89F4FFB52635C8F342E7FFB731D68E36C5982072A",
      "evidence_hash": "",
      "proposer_address": "B0DF524982C7F03C451135D0D6D53C9A8ECB0696"
    },
    "data": {
      "txs": [
        "lQLGwQI/Co0BHfNkigoUqaY1VPFyi8UqwLdCFw45IoPXpkoSFOMNfOPVbngQ19md/CoG1KJjhmBHGlt7ImZlZWRfcHJpY2UiOnsicHJpY2VzIjpbWyJ0ZXJyYTFjc3IyMnh2eHM2cjNna2pzbDdwbWprbXB0Mzltd2pzcm0wZTJyOCIsIjU0NzY0LjEyMDQ3MiJdXX19EhMKDQoEdXVzZBIFMjIxMzkQiYEJGmoKJuta6YchA7tLp2oZDmRColTBNeMPddpaIQ4C16BsgwW8mF/dwi7SEkAVr3CpWLTj818NSoLJX+VkQ94smX75xy1SAe6RvW0z1mVHSFp+EqcsymE7dwjvUrCiysVOdPhefg1yfGnM1A8y"
      ]
    },
    "evidence": {
      "evidence": null
    },
    "last_commit": {
      "height": "3565903",
      "round": "0",
      "block_id": {
        "hash": "7E62C40A1F5623534FFA698BB2A1652A30FE677A8201AC8A9D2197FE4B400188",
        "parts": {
          "total": "1",
          "hash": "CEB66523C6265F2377059C498F3F518FF22832B634CBFE4652BC040F9E1DDCBE"
        }
      },
      "signatures": [
        {
          "block_id_flag": 2,
          "validator_address": "04B605CB31E97AB840E37C4DC1AE04A266A49DE2",
          "timestamp": "2021-04-20T02:38:25.85715728Z",
          "signature": "/boFS+mWkhkSbldNCIbH9vq9te6AMwzcfWQuH6d2E78kE0SaP+5kI7K0OIls7puUS9UE1i3T5o70817XLNt7Cw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "090B0F80019F8BBC71FCE4C7D40C8353FB9C40E0",
          "timestamp": "2021-04-20T02:38:25.783672445Z",
          "signature": "FZR1DkR+oH12/flKsumDh1ofEml3MiRBr+2xwfx9kuH3w449yBvvb9ZaNHsPwP/YoORxpxnwd3bPs+le8Up5DA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "106F15C571906A8B9483D24BD79514F5071D565B",
          "timestamp": "2021-04-20T02:38:26.021876504Z",
          "signature": "26fZBPCnOueDom9dNP5KutMnginri/NRt8EpeoUOGMI8q6TpFI/pgyEdBsd7I1Vc4vFh6Ny3Fq+umI62S0OeDQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "118BC1F7C001FEA994E356A414CB6751E3F21D82",
          "timestamp": "2021-04-20T02:38:26.076917027Z",
          "signature": "4shUWONMCr2qLbc4S5FVdhBD9q0REtgz1yN1kA73ITDD0CWhgDfZxfjTdkcVoPEMB4BwZWvnmfDCdwFgxpZvBQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "168667A9304995C53FC6CF6773FD522B667EEB3A",
          "timestamp": "2021-04-20T02:38:26.066468486Z",
          "signature": "UZr2iUji8pXxW7Zey63jwZBAk3NRs7ATYCKPELUsUCvMvQ8TxBzsolfP0rjUkktrOna0AyO2bKDpcRt+C0EhCw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "26998468CBAB7F1520EEF1AC222D22270C0308AB",
          "timestamp": "2021-04-20T02:38:26.018002613Z",
          "signature": "vy5tbA5OT9oWHbxqh2laP7C9ahkHOOO0qK1e1of65KSRawKUEqtcwe6XLCi8yQoT1bjbotX9WemBWUcCfS4ICQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "342C1CF58474319236C4DB17077B0F160841BEEA",
          "timestamp": "2021-04-20T02:38:25.875728565Z",
          "signature": "STGOG/B32vbzn9x8ioSnOGov1T5qfh1ousDyVQClpXsJ8hAbwVh1I54Kc3zcV6bVdMHJ8fAM+/l71TaDA9JMAA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "3B783D4451B7F587A112BC393854D690B58C2096",
          "timestamp": "2021-04-20T02:38:26.063379471Z",
          "signature": "DTD35pgeZ7d4KoST8C9qvuw1rXkczItJ1+mpuBsVfyO6k/9sazJ95/BbUyW7A8Q/FSctwjZg33wvK6ylP3u0BA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "3D86088E9D238D4D408A5EA22842CF0C6063E122",
          "timestamp": "2021-04-20T02:38:25.996636049Z",
          "signature": "05gRTqSAiGsheWBr947+0h7oXlc3i/DBahDThbxrer9Y1Xwv3ZVr6Lb/ilo5aUJXeUoilsrNiARjYvmXLU7DAg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "42823B42D99BF70FCAF54CFA419D03789368EE88",
          "timestamp": "2021-04-20T02:38:25.803784472Z",
          "signature": "NMbmsG8rpwP80fVJBeftDTpopU+DdiMKnlcCMEbGSG9GqwNypY5EYmpCHNyqlTaL2QvhlUbOHw62ycwzLcqHBg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "47722D3A36065921188A1C93EBE471309078BDAE",
          "timestamp": "2021-04-20T02:38:25.927044241Z",
          "signature": "BvHXxJFb005yoyr24P4NcVSgxGFx39hXZ867uUbGhzkSjgmgNGLQqKb8C76w+c2kSTPXGnxd1xSb2DtQBf1XCQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "498A29151B47A82AFC18B84A0E64E525FAA59F8F",
          "timestamp": "2021-04-20T02:38:26.013698662Z",
          "signature": "EOxfkHMSldHmN0AEOVma2+9/iaPYrZF/pdPsemRpFx28eVekwM/Z67qbNC4vqbqmnIH8ja2KD2kwrdI6ulGAAQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "4AFD390FDA4DF4DD1DDA38470EC034989AC4276E",
          "timestamp": "2021-04-20T02:38:25.860858938Z",
          "signature": "Pps8LXU/ufKb0ZYWujbk9m9H6JGUjpQh33OEoTSUR1Vw8xurFuWpuIrOccEor1OSoDASZ7QHuynJLJZs+9GyBQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "4C3B572F2B5D8AD0A021868297597C4DA81A8288",
          "timestamp": "2021-04-20T02:38:26.027832399Z",
          "signature": "llyDMwWf+PHMKUv+OCoBi7xW8v0tWb0s0kyl/xNuBiN55y9MMkyttB6cQ+FFbXSnipqghj5CZqozX41+wHWGDg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "4DFC576869EBC0DA759D36C291A3A65539F1135D",
          "timestamp": "2021-04-20T02:38:25.984834271Z",
          "signature": "8WqkmTSlEudGZ1qNIUiEspmseLee2eOQsBvjMg/yC4lQlplvc5NUdoI49jq6eLo8a83ik4JziLJEme13jKHSCw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "504BD2871221290F190D43E2BF49EE2E2471F159",
          "timestamp": "2021-04-20T02:38:25.955331962Z",
          "signature": "fTDotNRon3OMbYzuJHaS48EqR1nEhh4A3xeVDkNEySdmj5/l6hRXWvMfAd0oxzssniB5VDi51t+Ux5i27dsgAA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "525BD01ACD7BC7D1FBE9B1D84EC691A08E60E427",
          "timestamp": "2021-04-20T02:38:25.92992251Z",
          "signature": "fRb0MuOJl3q6KkVjYG7kT2UKLpIOiY1N2WW8w5vOPYuUb9eGsDYUhlVhlyI5DMBIrI/QUMepaVQeE5KxoZNCBQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "5984C82797EB2DBC38B58C8AF65F6EC72A80A2F6",
          "timestamp": "2021-04-20T02:38:25.900215742Z",
          "signature": "ze1RHp47lmpt9Ix3NoZHP0PbMdZqogZ3t3A1zy8zeMYmMIJp7+r4ErqhCkLKMzrGdZxnsKghMXKW4TC+1ktlCg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "5F433107872C933B6952716F48F5DF7A153C117D",
          "timestamp": "2021-04-20T02:38:26.016880516Z",
          "signature": "ZgoOa9sK6KMKtz9/JtfXUYcdSB2o/za7dUgPeh3j9X//XMieHeA1N1I6aPPEQs2pwWFzqlxahkwSW3WjJjcxAQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "626C6C5872AAE26B615E84DFF6C2538B28F665BA",
          "timestamp": "2021-04-20T02:38:26.083853518Z",
          "signature": "PUgTqu/zNEu00UsoyQSX5LqmYRKh1L9ioBVnlXc7wwklLpQ3BH9mJnudA/2mOO7U9zFoRCBcsixpMhpN/hSQBw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "6CBA959413F5C9E50D3A96A3B2837FCD67315915",
          "timestamp": "2021-04-20T02:38:25.930940656Z",
          "signature": "LntCQZvhFSVKdPn78If1HDGh4AUbWpl0WyuXOhKAVhITF9+QoFbP1iOKw5clV9w35vKUzoJz8rdSahN+VnIxCA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "703BC9F4044957CA9CB9772987A7C3695B7268CC",
          "timestamp": "2021-04-20T02:38:25.970743156Z",
          "signature": "XI4Qxp5nAX8eFkhrLgGUSAYTEj9papj6Z7TnP6iqdAg4OniPmYhPYBF+W004kO/tcWW6wnTopO09WPbeX+9tDw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "72A42F7622B06B845583DAB0B881CE0EDF612450",
          "timestamp": "2021-04-20T02:38:26.162134135Z",
          "signature": "vPSKWad0yegyqlFaSTFZ6kIMJLLrrdwZ4TnKJQnPPSlhE4ayxt6AwDRgAV2bj7DkLMbCXVvZCDp52wu72tQpCA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "7BF1A4D3EE47DF78FC4D00EAE07CB9FA7DC6F800",
          "timestamp": "2021-04-20T02:38:26Z",
          "signature": "LlHOXNpJGHR/0obqK3kODVsfT9JQqahAeL2w70ciYlZMTdPO/NX43XwHk8Z4kM0jqaTKKVueIDpvxfCrFeQSCw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "7C3B55855981D65ABA786A5F82FE2D18F2C47E78",
          "timestamp": "2021-04-20T02:38:26.016895049Z",
          "signature": "nFtG0YEIk4qrDJor1QF4Nha2jth5E7O6r+3GAA5ZeEW9CYbNpuKmi4K/7HwPy6XtefyKvCqgRNJB2UWzmTp4Aw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "7F61D2F556C2545268594893E45E614F6DCE5423",
          "timestamp": "2021-04-20T02:38:25.802749459Z",
          "signature": "v9r820t5m2Ag2YfOQsy/Os8nvATSLEkkA0hEfaRU1k+MSFrEQnjb3OryrKE09o2tR1B+cncYXEorgKLVpKQKCQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "80012CC5EF82432C83A72B60BD7ED9237C915BAB",
          "timestamp": "2021-04-20T02:38:26.033322874Z",
          "signature": "PNs861erZf9+NcGPAyBkWi29p8fQZQjNDXDsju+5MKXuDnAe+4E9wXDjLeE8oc5wkTzlTBRw4NIEfMclLrqVAA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "831F402BDA0C9A3F260D4F221780BC22A4C3FB23",
          "timestamp": "2021-04-20T02:38:26.046559118Z",
          "signature": "kX5/z9AQ2TKvPIABnaD2r6V/hN5p3uAOo/8Vs4YyjnedL0i3Z2pVeMGIUMTQqU0/7gZhyNs9ML3lhWL5KnXTBQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "8A33294384015A285536BBC46BC69BE08C0F9041",
          "timestamp": "2021-04-20T02:38:26.047331125Z",
          "signature": "WE7SxaWrAH3lIGhZJIDln4EI3eK3lHbaXGsrp3f9HFJzgGrTuMBLBHY33SWaqeclKEGt6EsbBzO/AQk0eghnAw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "8AC9DAB9F3D14602F50C53BD065BCD9918EF2C64",
          "timestamp": "2021-04-20T02:38:26.08277435Z",
          "signature": "X9zS84s4ubKVBK5E6eb1+X8fG/O7qUU9OzrfdoEZaI34lQwKkt5CpYq4VcR1QjYCZTjTsYsUGHKnV1j5++JdCw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "8DDFF92020E013D815FCBDBC8C5070A10A7A7B90",
          "timestamp": "2021-04-20T02:38:25.86631622Z",
          "signature": "44W7IZgHrZUJfxchYKvY/OAJy4bmesPXPcTPoYprQ+QrvRbNAfmLIFUwcgxAGXQgD/dOwPmM+snuqLmFrehNCw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "9400554CB35D9A4A4F3CA1B33D9884309F4E0DCB",
          "timestamp": "2021-04-20T02:38:26.022103773Z",
          "signature": "4R1ejPjhccnNlpjqbqNCoyB8+jUSDf0087F+g4OQv9EIVR4K0hOOTD7E5LWa7oZBw/nCSE4g073haSV4j77gCA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "985688E4C0C4EA1546DA7740E5258C69DB624839",
          "timestamp": "2021-04-20T02:38:25.872792047Z",
          "signature": "BQXJR3UFLaUb265O078Wql+V6Ib52rAuP2xUsUJS83Ly8q5Nc4L2jXtuoNzRTy7sTj8wrlfDWXZjgg7aaovxDQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "9A6DEF1C88E4659D527D9DD6A24F6C1B89927493",
          "timestamp": "2021-04-20T02:38:25.790440312Z",
          "signature": "VJ+Jxy0J7Y4Nzi4sfh4R5XNPzAo42YpFiQMNnrzgmcVgR5eGJ7XKAuxdT7LDCNNBMno69Mp70ztehZCsffaxAA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "9D2428CBAC68C654BE11BE405344C560E6A0F626",
          "timestamp": "2021-04-20T02:38:26.055501236Z",
          "signature": "Z7k35HxOOcy/8dy53RQYAjUFLeo7If0l9Owd2u3GUrC8DZunpz5/j6Em6VLZfOUIGt8WcCly8hTmgvvY8JpfBQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "9E4E423ACBF7B3AD64DEB08244CA3CF7243D39D1",
          "timestamp": "2021-04-20T02:38:26.030919688Z",
          "signature": "UiSlb3UztN2lyQ4Zonm5oylRjSVaqLmOnW0l9VCSYqVrbDUZ+GF9SdBafAmJQbGZZDcvDkGXGvz3cLcpUty0BQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "A9E380D4D06ACF44A831AFBC2480D9305B88D0EB",
          "timestamp": "2021-04-20T02:38:25.915908126Z",
          "signature": "iOpfs9LbR74Yao3XV+IuyGpJ67zO+OwK3rP1cgfDJlZ5ZMHlwXU+o0haWXpRT3nJJ8t4EX1XtrtR/x2XGmKbCA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "AA1A027E270A2BD7AF154999E6DE9D39C5711DE7",
          "timestamp": "2021-04-20T02:38:26.034073015Z",
          "signature": "F3m6SL6al5SaPSUpNhvqGvxTz9dt7LRdW0dUyjTmXO3P7SLQR2HfEuxV5jQ30lkbtkU4NCebQAbl+5vwvAwQCQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "AF4B2849027F9307D7E89568FE4B56CA63944563",
          "timestamp": "2021-04-20T02:38:25.976354881Z",
          "signature": "g5A+GwqVPFoYPC0J8HdXsCmlZIhOe5QD+u6oG2049dm6QlP5wU7W0mVOv1GXekuoaIZNLx3VgYzNOFIVSFd2Dw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "B0DF524982C7F03C451135D0D6D53C9A8ECB0696",
          "timestamp": "2021-04-20T02:38:25.774184604Z",
          "signature": "wf96+fPzOuhy0UVysbnqTjXOmYHjJy5813aqM/GSKclVEdkQvFenaMI0jZT3ftnqxqU85t07pbbbcvyUlgaBCw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "B3A35446E08F1B75AC23B5E3DBC1EFEDE6C827A4",
          "timestamp": "2021-04-20T02:38:25.870514239Z",
          "signature": "v7b907/m2VNgYmPBhznAsMPW7LC4sVXeqPHEbixHk/5u2ZnnJZKxF1ZyABs0xuEHs1C8GiYrWayTNqQ5jrk4CA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "BB0C80D81C29173349C025A4EA8C337E3B0E243A",
          "timestamp": "2021-04-20T02:38:25.934996139Z",
          "signature": "Tvua/LgAagk8vkv2N01Ky26EY1WDyZs+mHG7jAAuTH/lzoRAU6au+ExV1wbre8YxYMwDqH5zDmHumTr1xsTTAA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "BF2C6BD7D83BDA17FFE502C54135722974D65FE7",
          "timestamp": "2021-04-20T02:38:26.01127536Z",
          "signature": "CahSoJHpaesolwUajY2FdtMmOcHk4O/zB0lr8j2fpOAeYGmJnsS48LhJZD+YCXIsyPnmF6/RlLolL/xpuaNcDg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "BF546DA9DEDDCD5EE920981E2B518E12993CC22B",
          "timestamp": "2021-04-20T02:38:25.927930721Z",
          "signature": "4Rb6uTkA65e11fs1N+jVvqE561QvpEMRFd1wccqTVmrYmAopmz1H1P8tDLlT/RNx/N14yx76CGoEth9CzRhoAg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "D088AA1191A6751668BCE4282C538AC68871547D",
          "timestamp": "2021-04-20T02:38:25.89057898Z",
          "signature": "e3fuFCielpFSXoDc3EF3WvDw4b+sBkMbIdT9KmPgy30TfH474E3uPC/xdlHvENdRzaHidZsPyPot6tGIEyzaDw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "D36A5D20F5A3B48FAFED48E3674C0FE452F76DF6",
          "timestamp": "2021-04-20T02:38:25.989266921Z",
          "signature": "Whl6pDlg9/31+9t7x3mMie6qytQ0bWqQT81F0sOcql/XDqcXCzmC447Q2AsN6FLDJL7AodcVaK31CARi6lAUDw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "D42AD28B7ED12126D38366E8C474F85B34ED4269",
          "timestamp": "2021-04-20T02:38:26.036587721Z",
          "signature": "Zmfk0m6XZPLID1otgn7g1VosOLVpnSKc26jxWWuHlRG5UW9n71g8aBsAeGkkqLP2k+lAlO7EXLefqCCi4tzHAA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "D921686AA5B80147FC2AD08C0EB242B5149AABA2",
          "timestamp": "2021-04-20T02:38:26.044280233Z",
          "signature": "SCfYrDayka3kd1EZUIHPGDs5UiQLzwuuaXMKy1qJnfViohPCzFsK1FffFK+1jmW78BNhdxHUshqO+wppZ60NAQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "D9591B899BF737BAA3E6475C2EB395865C70DDC4",
          "timestamp": "2021-04-20T02:38:26.014435437Z",
          "signature": "/cTWu5GG7r03xlmzfcsYFxcDD/sWnneBm4KlMMzrTdISvTkxp/JJ8xoBS8whsHisC5CYdCRNtiTLJlhPYpXVCQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "DC9897F22E74BF1B66E2640FA461F785F9BA7627",
          "timestamp": "2021-04-20T02:38:26.014575852Z",
          "signature": "alYzuDpCmg1DOX8C/0wkKNd/32g5jZepJWpeei0Pn3eKKrEiNY2SVX7mn9oe3Yv9nd1+mvH7GSm30MN0S4HhDA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "E0F8D500C9D835D0452C16E8D9D2D9E04FF75120",
          "timestamp": "2021-04-20T02:38:26.053131104Z",
          "signature": "p1E4dmiv0CNN6zlWvjgLdYHQhdxcc4oUJsrXhz+MaCNztwrQg3FND8/hoBjv5/1MA0416HwIYWxaKYidlAbODg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "E132B05B442540F958BD32E70148BBD29A517D95",
          "timestamp": "2021-04-20T02:38:25.927388494Z",
          "signature": "y+SbcZfi5UhQZN1jG31lMdHsPQXGhxW2n+fCGzllo2sPyFpZlH/PBhLy6V8CR/Pju9PLNN2gTu8gKNeHonJCBA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "E7926032644085F6CA47601D6D04CAC569C91099",
          "timestamp": "2021-04-20T02:38:25.893878334Z",
          "signature": "gpyo59jDLYnBr6q1gBUJEnYB4eoNkIxFYJCEZzqJELOnrNStrKnJDoH/kY4DjjMSk9sq2hWHa3TQeSJmeNlSCA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "EEA4891F5F8D523A6B4B3EAC84B5C08655A00409",
          "timestamp": "2021-04-20T02:38:26.022323247Z",
          "signature": "T4iVg7nCBtTAc8N7N0A6kFqvIFBOFZW0CyXfESVycYJI0DdmYStX3CHnNltD6hSPY8I2qRWdEAQGSNXZCKc+AQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "EF949FC9395DBD02E085294D39478F49255926AE",
          "timestamp": "2021-04-20T02:38:25.990994307Z",
          "signature": "gr+WgEb76ExU9/WV/e/vB1P0+hmK2vJPm/DxC2/edwK9LwsFHjJnErnbBJekwI1OzZL9pZ3B6eKdrx6BcaeyCw=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "F2683F267D2B4C8714B44D68612DB37A8DD2EED7",
          "timestamp": "2021-04-20T02:38:25.937837202Z",
          "signature": "c7N+NPlFC+nudwfxknXc55P/QAYxIQymSoGLio+DrErHkOewxZVAdGWoqQFSkbeWLXGn2C5/eqx2WhoDydQ2Dg=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "F3C8D78CEA57B74A628ADF09CAF6D87DDD333142",
          "timestamp": "2021-04-20T02:38:25.886307811Z",
          "signature": "vP7EQhdvYFX8rX5jPEzOmlTnCaT+1/Va4HQNA4/G9y72ZTyoS2u5s6bqNh6yXrSzBliZWxLqtDRkesgSoqtZCA=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "F63A8D51F57631BBE165052137471F9F1542132E",
          "timestamp": "2021-04-20T02:38:26.058897421Z",
          "signature": "p03yFZfaFGZ+X2PEPIWBm69l0jigrUxCO+wndbL3cL4vRSoCkmAirGWKlJ09vHy/mi200FbxGlzRvXO9sAbDBQ=="
        },
        {
          "block_id_flag": 2,
          "validator_address": "FE24E00027858FAFF5086F50CB2BF913D91BE7E9",
          "timestamp": "2021-04-20T02:38:26.021539353Z",
          "signature": "U2lkq8upHqUIb2Pkxhl1r0OciCN5OhA6dYJZnWwQI0PcuHgN0SBCkSzZANEqN82s9NTP0Lx0v2QkYL30O+QUDA=="
        }
      ]
    }
  }
}

```

GET /auth/accounts/terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v

```json
{
  "height": "3565904",
  "result": {
    "type": "core/Account",
    "value": {
      "address": "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v",
      "coins": [
        {
          "denom": "uaud",
          "amount": "338"
        },
        {
          "denom": "ucad",
          "amount": "427"
        },
        {
          "denom": "uchf",
          "amount": "51"
        },
        {
          "denom": "ueur",
          "amount": "1167"
        },
        {
          "denom": "ugbp",
          "amount": "379"
        },
        {
          "denom": "uhkd",
          "amount": "2253"
        },
        {
          "denom": "uinr",
          "amount": "501"
        },
        {
          "denom": "ujpy",
          "amount": "242781"
        },
        {
          "denom": "ukrw",
          "amount": "30611166781"
        },
        {
          "denom": "uluna",
          "amount": "1430057312"
        },
        {
          "denom": "umnt",
          "amount": "38006560"
        },
        {
          "denom": "usdr",
          "amount": "116610"
        },
        {
          "denom": "usgd",
          "amount": "9"
        },
        {
          "denom": "uthb",
          "amount": "14328"
        },
        {
          "denom": "uusd",
          "amount": "3040770311"
        }
      ],
      "public_key": {
        "type": "tendermint/PubKeySecp256k1",
        "value": "AjszqFJDRAYbEjZMuiD+ChqzbUSGq/RRu3zr0R6iJB5b"
      },
      "account_number": "1165",
      "sequence": "2642"
    }
  }
}
```

POST /txs
Content-Type: application/json;charset=utf-8
body
```json
{
  "tx": {
    "msg": [
      {
        "type": "oracle/MsgAggregateExchangeRatePrevote",
        "value": {
          "hash": "e667151e8b33437fb5fb5738b22cd6d4b2af424d",
          "feeder": "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v",
          "validator": "terravaloper1usws7c2c6cs7nuc8vma9qzaky5pkgvm2ujy8ny"
        }
      }
    ],
    "fee": {
      "gas": "1000000",
      "amount": []
    },
    "signatures": [
      {
        "signature": "IOmkDgZWe0Ebc0TFV/4VZI0V83uuNflf1Ree2MADOYt+edhNjnRRf67wm2rx4wxpz2W0xB1nV+o7+VRK7MC5qg==",
        "pub_key": {
          "type": "tendermint/PubKeySecp256k1",
          "value": "AjszqFJDRAYbEjZMuiD+ChqzbUSGq/RRu3zr0R6iJB5b"
        }
      }
    ],
    "memo": "@terra-money/oracle-feeder@1.3.4"
  },
  "mode": "async"
}
```
Response
```json
{
  "height": "0",
  "txhash": "D59030EAC8FA1277D3567E5B9FA80D25DFF8FE9C52DFE060430A5418984BDDCE"
}

```

GET /blocks/latest
...

GET /txs/D59030EAC8FA1277D3567E5B9FA80D25DFF8FE9C52DFE060430A5418984BDDCE 
```json
{
  "error": "Tx: RPC error -32603 - Internal error: tx (D59030EAC8FA1277D3567E5B9FA80D25DFF8FE9C52DFE060430A5418984BDDCE) not found"
}
```
