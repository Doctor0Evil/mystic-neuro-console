import kotlinx.coroutines.channels.Channel
import java.net.URI

fun main() {
    val incoming = Channel<CommandResult>(capacity = 16)
    val client = RustWsClient(URI("ws://127.0.0.1:8080"), incoming)
    client.connectBlocking()

    val orchestrator = ClusterOrchestrator(client, incoming)
    orchestrator.start()
}
