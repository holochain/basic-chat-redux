module.exports = scenario => {

  const { config1, config2, config3, config4 } = require('../config')

  const convoHoloscape = {
    name: 'Holoscape rox!',
    description: '',
    initial_members: [],
    public: true
  }

  const convoHoloscapeMessage1 = {
    timestamp: 0,
    message_type: 'text',
    payload: 'This is Philip chatting in the Holoscape conversation...',
    meta: '{}',
  }

  const convoHoloscapeMessage2 = {
    timestamp: 0,
    message_type: 'text',
    payload: 'This is Willem chatting in the Holoscape conversation...',
    meta: '{}',
  }

  const convoHoloscapeMessage3 = {
    timestamp: 0,
    message_type: 'text',
    payload: 'Phil posting again in the Holoscape conversation...',
    meta: '{}',
  }


  scenario('4 Agents chatting', async (s, t) => {
    const {player1} = await s.players({player1: config1}, false)
    await player1.spawn()
    await player1.call('chat', 'chat', 'register', {name: 'player1', avatar_url: ''})
    await s.consistency() 

    const create_result = await player1.call('chat', 'chat', 'start_conversation', convoHoloscape)
    await s.consistency()
    console.log(create_result)
    const post_result = await player1.call('chat', 'chat', 'post_message', {conversation_address: create_result.Ok, message: convoHoloscapeMessage1})
    await s.consistency()
    console.log(post_result)
    t.notEqual(post_result.Ok, undefined, 'post should return Ok')

    const {player2} = await s.players({player2: config1}, false)
    await player2.spawn()
    await player2.call('chat', 'chat', 'register', {name: 'player2', avatar_url: ''})
    await s.consistency() 

    const public_conversations_result = await player2.call('chat', 'chat', 'get_all_public_conversations', {})
    console.log(public_conversations_result.Ok)
    const holoscape_convo_address = public_conversations_result.Ok[0].address
    await player2.call('chat', 'chat', 'join_conversation', {conversation_address: holoscape_convo_address})
    await s.consistency()
    const player2_post_result = await player2.call('chat', 'chat', 'post_message', {conversation_address: holoscape_convo_address, message: convoHoloscapeMessage2})
    await s.consistency()
    console.log(player2_post_result)
    t.notEqual(player2_post_result.Ok, undefined, 'post should return Ok')
    const get_message_result = await player2.call('chat', 'chat', 'get_messages', {address: holoscape_convo_address})
    await s.consistency()
    console.log(get_message_result)
    t.deepEqual(get_message_result.Ok.length, 2, 'a message from player1 and player2')

    const {player3} = await s.players({player3: config1}, false)
    await player3.spawn()
    await player3.call('chat', 'chat', 'register', {name: 'player3', avatar_url: ''})
    await s.consistency() 

    const player3_public_conversations_result = await player3.call('chat', 'chat', 'get_all_public_conversations', {})
    console.log(player3_public_conversations_result.Ok)
    const player3_holoscape_convo_address = player3_public_conversations_result.Ok[0].address
    await player3.call('chat', 'chat', 'join_conversation', {conversation_address: player3_holoscape_convo_address})
    await s.consistency()
    const player3_get_message_result = await player3.call('chat', 'chat', 'get_messages', {address: player3_holoscape_convo_address})
    await s.consistency()
    console.log(player3_get_message_result)
    t.deepEqual(player3_get_message_result.Ok.length, 2, 'Player 3 sees messages from player1 and player2')

    const player1_post_result_2 = await player1.call('chat', 'chat', 'post_message', {conversation_address: holoscape_convo_address, message: convoHoloscapeMessage3})
    await s.consistency()

    const player3_get_message_result_2 = await player3.call('chat', 'chat', 'get_messages', {address: holoscape_convo_address})
    await s.consistency()
    console.log(player3_get_message_result_2)
    t.deepEqual(player3_get_message_result_2.Ok.length, 3, 'Player 3 sees 2 messages from player1 and 1 from player2')

    const {player4} = await s.players({player4: config1}, false)
    await player4.spawn()
    await player4.call('chat', 'chat', 'register', {name: 'player4', avatar_url: ''})
    await s.consistency() 
 
    const player4_public_conversations_result = await player4.call('chat', 'chat', 'get_all_public_conversations', {})
    console.log(player4_public_conversations_result.Ok)
    const player4_holoscape_convo_address = player4_public_conversations_result.Ok[0].address
    await player4.call('chat', 'chat', 'join_conversation', {conversation_address: player4_holoscape_convo_address})
    await s.consistency()
    const player4_get_message_result = await player4.call('chat', 'chat', 'get_messages', {address: player4_holoscape_convo_address})
    await s.consistency()
    console.log(player4_get_message_result)
    t.deepEqual(player4_get_message_result.Ok.length, 3, 'Player 4 sees 2 messages from player1 and 1 from player2')
  })
}
