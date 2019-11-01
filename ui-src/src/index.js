import React from 'react'
import ReactDOM from 'react-dom'
import { connect } from '@holochain/hc-web-client'
import { Group } from './components/Group'
import './index.css'
// --------------------------------------
// Application
// --------------------------------------
const REACT_APP_WEBSOCKET_INTERFACE = process.env.REACT_APP_WEBSOCKET_INTERFACE  //'ws://localhost:10000' //
const INSTANCE_ID = "basic-chat";

export class View extends React.Component {
  constructor (props) {
    super(props)
      this.state = {
        holochainConnection: connect({ url: REACT_APP_WEBSOCKET_INTERFACE }),
        connected: false,
        user: {},
        users: {},
        conversation: {},
        conversations: [],
        messages: {},
        sidebarOpen: false,
        userListOpen: window.innerWidth > 1000,
      }

    this.actions = {
      // --------------------------------------
      // UI
      // --------------------------------------

      setSidebar: sidebarOpen => this.setState({ sidebarOpen }),
      setUserList: userListOpen => this.setState({ userListOpen }),

      // --------------------------------------
      // User
      // --------------------------------------

      setUser: user => {
        this.setState({ user })
        this.actions.getConversations()
      },

      // --------------------------------------
      // Conversation
      // --------------------------------------

      setConversation: conversation => {
        this.setState({ conversation, sidebarOpen: false })
        this.actions.getMessages(conversation.id)
        this.actions.getConversationMembers(conversation.id)
        this.actions.scrollToEnd()
      },

      joinConversation: conversation => {
        console.log('joining conversation')
        this.actions.setConversation(conversation)
        this.makeHolochainCall(INSTANCE_ID + '/chat/join_conversation', { conversation_address: conversation.id }, (result) => {
          console.log('joined conversation', result)
        })
      },

      getConversationMembers: conversationId => {
        this.makeHolochainCall(INSTANCE_ID + '/chat/get_members', {
          conversation_address: conversationId
        }, (result) => {
          console.log('retrieved members', result)
          const users = result.Ok
          users.forEach(address => {
            this.actions.getUserProfile(address)
          })
          this.setState({
            conversation: { ...this.state.conversation, users }
          })
        })
      },

      sendMessage: ({ text, conversationId }) => {
        const message = {
          message_type: 'text',
          timestamp: Math.floor(Date.now() / 1000),
          payload: text,
          meta: ''
        }
        this.makeHolochainCall(INSTANCE_ID + '/chat/post_message', {
          conversation_address: conversationId,
          message
        }, (result) => {
          console.log('message posted', result)
          this.actions.getMessages(conversationId) // hack for now
        })
      },

      getMessages: (conversationId) => {
        this.makeHolochainCall(INSTANCE_ID + '/chat/get_messages', { address: conversationId }, (result) => {
          console.log('retrieved messages', result)

          const conversationMessages = result.Ok.map(({ address, entry }) => ({
            text: entry.payload,
            sender: entry.author,
            createdAt: entry.timestamp,
            id: address
          }))

          this.setState({
            messages: { ...this.state.messages, [conversationId]: conversationMessages }
          })
          this.actions.scrollToEnd()
        })
      },

      startConversation: options => {
        console.log(this.state.group)
        const conversationSpec = {
          name: options.name,
          description: '',
          initial_members: []
        }
        this.makeHolochainCall(INSTANCE_ID + '/chat/start_conversation', conversationSpec, (result) => {
          console.log('created conversation', result)
          this.actions.setConversation({
            id: result.Ok,
            name: options.name,
            users: []
          })
          this.actions.getConversations()
        })
      },

      getUserProfile: userId => {
        this.makeHolochainCall(INSTANCE_ID + '/chat/get_member_profile', { agent_address: userId }, (result) => {
          console.log('retrieved profile', result)
          this.setState({
            users: { ...this.state.users, [userId]: result.Ok }
          })
        })
      },

      getConversations: () => {
        this.makeHolochainCall(INSTANCE_ID + '/chat/get_all_public_conversations', {}, (result) => {
          console.log('retrieved public conversations', result)
          let conversations = result.Ok.map(({ address, entry }) => {
            return {
              id: address,
              private: !entry.public,
              name: entry.name,
              users: []
            }
          })
          this.setState({
            conversations
          })
        })
      },

      registerUser: ({ name, avatarURL }) => {
        this.makeHolochainCall(INSTANCE_ID + '/chat/register', { name, avatar_url: avatarURL }, result => {
          console.log('registered user', result)
          this.actions.setUser({ id: result.Ok, name, avatarURL })
        })
      },

      scrollToEnd: e =>
        setTimeout(() => {
          const elem = document.querySelector('#messages')
          elem && (elem.scrollTop = 100000)
        }, 0)

    }
  }

  handleSignal(signal) {
    console.log(JSON.stringify(signal.signal))
    if (signal.signal.name === 'new_message') {
      const {conversationId} = JSON.parse(signal.signal.arguments)
      // this can fail because the get might not be able to see the new message
      this.actions.getMessages(conversationId)
    }
  }

  componentDidMount () {
    this.state.holochainConnection.then(({ callZome, call, onSignal }) => {
      console.log('holochainConnection')
      this.setState({ connected: true })
      onSignal(this.handleSignal)
      this.makeHolochainCall(INSTANCE_ID + '/chat/get_my_member_profile', {}, (result) => {
        const profile = result.Ok
        console.log('result:', result)
        if (profile) {
          console.log('registration user found with profile:', profile)
          this.actions.setUser({ id: profile.address, name: profile.name, avatarURL: profile.avatar_url })
        }
        else {
          console.log('User has not registered a profile. Awaiting registration')
        }
      })
    })
  }

  makeHolochainCall (callString, params, callback) {
    const [instanceId, zome, func] = callString.split('/')
    this.state.holochainConnection.then(({ callZome }) => {
      callZome(instanceId, zome, func)(params).then((result) => callback(JSON.parse(result)))
    })
  }

  render () {
    let props = {
      ...this.state,
      ...this.actions,
    }

    return (
      <Group {...props} />
    )
  }
}

ReactDOM.render(<View />, document.querySelector('#root'))
