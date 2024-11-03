$tcpClient = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 8080)
$stream = $tcpClient.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

$writer.WriteLine("Hello From Powershell")
$writer.Flush()

$response = $reader.ReadLine()
Write-Output "Response: $response"

$writer.Close()
$reader.Close()
$tcpClient.close()

