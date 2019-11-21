const { Orchestrator, tapeExecutor, singleConductor, localOnly, combine, callSync  } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  console.error('got unhandledRejection:', error);
});


const networkType = process.env.TEST_NETWORK_TYPE
const middleware = 
  ( networkType === 'websocket'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim1h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim2h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'memory'
  ? combine(tapeExecutor(require('tape')), localOnly, singleConductor, callSync)

  : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
)

const orchestrator = new Orchestrator({
  middleware,
  waiter: {
    softTimeout: 10000,
    hardTimeout: 20000
  }
})

require('./agent/messages')(orchestrator.registerScenario)
// require('./scenario/4-agents')(orchestrator.registerScenario)

orchestrator.run()
