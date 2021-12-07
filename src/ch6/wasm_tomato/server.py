import http.server, socketserver

# MIMEにapplication/wasmを追加
Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map['.wasm'] = 'application/wasm'

# サーバーを起動
port = 8888
with socketserver.TCPServer(("", port), Handler) as d:
    print("[起動] http://localhost:{}".format(port))
    try:
        d.serve_forever()
    except:
        pass
    finally:
        print("close")
        d.server_close()

