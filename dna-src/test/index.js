const path = require('path')
const tape = require('tape')
const { Orchestrator, combine, callSync, tapeExecutor } = require('@holochain/try-o-rama')
const orchestrator = new Orchestrator({
  globalConfig: {
  	logger: true, 
  	network: 'memory',
  },
  middleware: combine(callSync, tapeExecutor(tape))
})
process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

require('./agent/messages')(orchestrator.registerScenario)
// require('./scenario/4-agents')(orchestrator.registerScenario)

orchestrator.run()
