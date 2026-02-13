const { init } = require('../index.js')

try {
    const client = init(480)
    const friends = client.friends

    console.log('Testing Friends API...')
    
    // Test FriendFlags existence
    if (!friends.FriendFlags) {
        throw new Error('FriendFlags is missing')
    }
    console.log('✓ FriendFlags found')

    // Test getFriends
    const friendList = friends.getFriends(friends.FriendFlags.Immediate)
    console.log(`✓ getFriends returned ${friendList.length} friends`)

    if (friendList.length > 0) {
        const firstFriend = friendList[0]
        
        // Test Friend object structure
        if (typeof firstFriend.steamId !== 'bigint' || typeof firstFriend.name !== 'string') {
            throw new Error('Invalid Friend object structure')
        }
        console.log('✓ Friend object structure is valid')

        // Test getFriendPersonaName
        const personaName = friends.getFriendPersonaName(firstFriend.steamId)
        if (personaName !== firstFriend.name) {
            throw new Error(`Persona name mismatch: expected ${firstFriend.name}, got ${personaName}`)
        }
        console.log(`✓ getFriendPersonaName returned correct name: ${personaName}`)
    } else {
        console.log('! No friends found to test individual properties, but getFriends call succeeded.')
    }

    console.log('All tests passed successfully!')
    process.exit(0)
} catch (err) {
    console.error('Test failed:', err)
    process.exit(1)
}
