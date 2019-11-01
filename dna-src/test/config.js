const { Config } = require('@holochain/try-o-rama')
const peer_chat_dnaPath = "./dist/dna-src.dna.json"
const peer_chat_dna_1 = Config.dna(peer_chat_dnaPath, 'chat_1')
const peer_chat_dna_2 = Config.dna(peer_chat_dnaPath, 'chat_2')
const peer_chat_dna_3 = Config.dna(peer_chat_dnaPath, 'chat_3')
const peer_chat_dna_4 = Config.dna(peer_chat_dnaPath, 'chat_4')

module.exports = {
  config1: {
    instances: {
      chat: peer_chat_dna_1,
    }
  },
  config2: {
    instances: {
      chat: peer_chat_dna_2,
    }
  },
  config3: {
    instances: {
      chat: peer_chat_dna_3,
    }
  },
  config4: {
    instances: {
      chat: peer_chat_dna_4,
    }
  }
}
