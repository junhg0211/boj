function excludeConnection(connection, startNode) {
    if (!connection.includes(startNode)) return;

    let queue = [startNode];

    while (queue.length > 0) {
        let now = queue[0];
        queue = queue.splice(0, 1);
    }
}
