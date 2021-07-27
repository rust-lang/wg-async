# 😱 Status quo stories: Alan builds a task scheduler

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

A core component of [`DistriData`][distridata], called `TaskScheduler`, is in charge of (1) receiving client requests via an HTTP server, (2) serializing them in a task queue, (3) relaying each task to the state machine applier (e.g., apply change to the storage backend), and (4) returning the result back to the client.

`TaskScheduler` was originally implemented in Go. New to Rust, [Alan][] believes Rust could provide the same quality of service but with less memory. Then decides to reimplement `TaskScheduler` in Rust, without knowing the challenges ahead.

Alan only read the first few chapters of [Rust book](https://doc.rust-lang.org/nightly/book/title-page.html) to understand the core concepts like ownership model and syntax. Already proficient in Go, Alan jumped into the coding by working through a hands-on project. Alan often referred to the examples found in each Rust crate but may lack deep understanding of how Rust works. Alan first focused on translating the Go code to Rust and as a result, the first iteration may be filled with non-idiomatic Rust code.

### **Implementing request ID generator**

Alan first transliterates request ID generator code, originally written in Go:

```go
import "sync/atomic"

type Generator interface {
	next() uint64
}

type generator struct {
	prefix uint64
	suffix uint64
}

func (gen *generator) next() uint64 {
	suffix := atomic.SwapUint64(&gen.suffix, gen.suffix+1)
	id := gen.prefix | (suffix & (math.MaxUint64 >> 16))
	return id
}
```

Alan learns Rust trait as the closest concept to Go interface but is now torn between [std::sync::atomic](https://doc.rust-lang.org/std/sync/atomic) and [crossbeam::atomic::AtomicCell](https://docs.rs/crossbeam). Reading multiple articles about how great crossbeam is and for its thread-safety promises, Alan chooses crossbeam (see ["crates better than std (from Reddit)"](https://www.reddit.com/r/rust/comments/hat5bt/what_are_your_favorite_better_than_std_crates/)):

```rust
use crossbeam::atomic::AtomicCell;

pub struct Generator {
    prefix: u64,
    suffix: AtomicCell<u64>,
}

impl Generator {
    pub fn new(...) -> Self {
        ...
    }

    pub fn next(&self) -> u64 {
        let suffix = self.suffix.fetch_add(1);
        let id = self.prefix | (suffix & (u64::MAX >> 16));
        id
    }
}
```

Accustomed to an opinionated way of doing concurrency in Go, Alan loses confidence in Rust async support, as he sees fragmented but specialized solutions in Rust async ecosystem.

### **Implementing event notifier**

Alan then implements the notifier to propagate the request and apply the progress with the scheduler and low-level state machine. In Go, it can be simply implemented as below:

```go
type Notifier interface {
	register(id uint64) (<-chan string, error)
	trigger(id uint64, x string) error
}

type notifier struct {
	mu       sync.RWMutex
	requests map[uint64]chan string
}

func (ntf *notifier) register(id uint64) (<-chan string, error) {
	ntf.mu.Lock()
	defer ntf.mu.Unlock()
	ch := ntf.requests[id]
	if ch != nil {
		return nil, fmt.Errorf("dup id %x", id)
	}

	ch = make(chan string, 1)
	ntf.requests[id] = ch
	return ch, nil
}

func (ntf *notifier) trigger(id uint64, x string) error {
	ntf.mu.Lock()
	ch, ok := ntf.requests[id]
	if ch == nil || !ok {
		ntf.mu.Unlock()
		return fmt.Errorf("request ID %d not found", id)
	}
	delete(ntf.requests, id)
	ntf.mu.Unlock()
	ch <- x
	close(ch)
	return nil
}
```

Alan now needs the equivalent to Go `sync.RWMutex`, and found multiple options:

- [`std::sync::RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
- [`parking_lot::RwLock`](https://docs.rs/parking_lot)

Already losing confidence in Rust std, Alan instead chooses `parking_lot`, as it claims up to 5x faster performance than `std::sync::Mutex` (see [github](https://github.com/Amanieu/parking_lot#parking_lot)). After numeruous hours of trials and errors, Alan discovered that `parking_lot::RwLock` is not intended for async/future environments (see [github issue](https://github.com/Amanieu/parking_lot/issues/86)). Having to think about which library to use for thread and async programming, Alan appreciates the simplicity of Go concurrency where threads are effectively abstracted away from its users. Alan is now using [`async_std::sync::RwLock`](https://docs.rs/async-std/1.9.0/async_std/sync/struct.RwLock.html) which seems nicely integrated with Rust async programming.

To send and receive events, Alan needs the equivalent of Go channel but is not sure about [`std::sync::mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html), as he sees two other options: [Flume](https://github.com/zesterer/flume) which claims to be much faster than std (see ["Flume, a 100% safe MPSC that's faster than std (from Reddit)"](https://www.reddit.com/r/rust/comments/fj17z6/flume_a_100_safe_mpsc_thats_faster_than_std_and/)), and [`crossbeam_channel`](https://docs.rs/crossbeam-channel/0.5.1/crossbeam_channel/). Having used crossbeam, Alan chose crossbeam channel:

```rust
use async_std::sync::RwLock;
use crossbeam_channel::{self, unbounded};

pub struct Notifier {
    requests: RwLock<HashMap<u64, crossbeam_channel::Sender<String>>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            requests: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, id: u64) -> io::Result<crossbeam_channel::Receiver<String>> {
        let mut _mu;
        match self.requests.try_write() {
            Some(guard) => _mu = guard,
            None => return Err(...),
        }

        let (request_tx, request_rx) = unbounded();
        if _mu.get(&id).is_none() {
            _mu.insert(id, request_tx);
        } else {
            return Err(...)
        }

        Ok(request_rx)
    }

    pub fn trigger(&self, id: u64, x: String) -> io::Result<()> {
        let mut _mu;
        match self.requests.try_write() {
            Some(guard) => _mu = guard,
            None => return Err(...),
        }

        let request_tx;
        match _mu.get(&id) {
            Some(ch) => request_tx = ch,
            None => return Err(...),
        }

        match request_tx.send(x) {
            Ok(_) => _mu.remove(&id),
            Err(e) => return Err(...),
        }

        Ok(())
    }
}
```

Alan is still not sure if `crossbeam_channel` is safe for async programming and whether he should instead use another crate [`async_std::channel`](https://docs.rs/async-std/1.9.0/async_std/channel/index.html). While `crossbeam_channel` seems to work, Alan is not confident about his choice. Disgruntled with seemingly unnecessary divergence in the community, Alan wonders why all those cool improvements had not been made back to Rust core std libraries.

### **Implementing task applier**

Alan implements a task applier, which simply echoes the requested message, as in Go:

```go
type EchoManager interface {
	apply(req *EchoRequest) (string, error)
}

type echoManager struct {
	mu sync.RWMutex
}

func (ea *echoManager) apply(req *EchoRequest) (string, error) {
	ea.mu.Lock()
	defer ea.mu.Unlock()
	switch req.Kind {
	case "create":
		return fmt.Sprintf("SUCCESS create %q", req.Message), nil
	case "delete":
		return fmt.Sprintf("SUCCESS delete %q", req.Message), nil
	default:
		return "", fmt.Errorf("unknown request %q", req)
	}
}
```

Having implemented event notifier above, Alan is now somewhat familiar with Rust mutex and writes the following Rust code:

```rust
// 1st version
use async_std::sync::RwLock;

pub struct Manager {
    mu: RwLock<()>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            mu: RwLock::new(()),
        }
    }

    pub fn apply(&self, req: &Request) -> io::Result<String> {
        let _mu;
        match self.mu.try_write() {
            Some(guard) => _mu = guard,
            None => return Err(...),
        }
        match req.kind.as_str() {
            "create" => Ok(format!(
                "SUCCESS create {}",
                to_string(req.message.to_owned())
            )),
            "delete" => Ok(format!(
                "SUCCESS delete {}",
                to_string(req.message.to_owned())
            )),
            _ => Err(...),
        }
    }
}
```

The code compiles and thus must be safe. However, after reviewing the code with [Barbara][], Alan learns that while `std::sync::Mutex` protects data from concurrent access, `std::sync::Mutex` itselt must be also protected between threads. And the code will not compile if he tries to use it from multiple threads. This is where [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) comes in to provide safe multi-threaded access to the `Mutex`.

[`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) documentation explains `Arc` in depth. If Alan had chosen `std::sync::Mutex` library, he would have known about `Arc`. Because Alan was initially given multiple alternatives for mutex, he overlooked the documentation in `std::sync::Mutex` and instead used [`async_std::sync::RwLock`](https://docs.rs/async-std/1.9.0/async_std/sync/struct.RwLock.html) whose documentation did not explain `Arc`. As a result, Alan did not know how to properly use mutex in Rust.

Deeply confused, Alan made a quick fix to wrap `Mutex` with `Arc`:

```rust
// 2nd version
use async_std::{sync::Arc, sync::RwLock};

pub struct Manager {
    mu: Arc<RwLock<()>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            mu: Arc::new(RwLock::new(())),
        }
    }
    ...
```

This raises multiple questions for Alan:

1. If `Mutex` itself had to be protected, why `Arc` is not unified into a single type? Is the flexibility of having different types really worth the less safety guarantee?
2. Rust claims unparalleled safety. Is it still true for async programming? Rust compiler not complaining about the missing `Arc` means `Mutex` is still safe without `Arc`?
3. What happens if the code went into production without `Arc`? Would the code have race conditions?
4. Does having `Arc` make code slower? Did I just introduce extra runtime cost?
5. Which one is safe for async programming: `std::sync::Arc` and `async_std::sync::Arc`?

### **Implementing task scheduler**

Alan then implements the task scheduler that calls event notifier and task applier above, as in Go:

```go
type Request struct {
	echoRequest *EchoRequest
}

type Applier interface {
	start()
	stop() error
	apply(req Request) (string, error)
}

type applier struct {
	requestTimeout time.Duration

	requestIDGenerator Generator
	notifier           Notifier

	requestCh chan requestTuple

	stopCh chan struct{}
	doneCh chan struct{}

	echoManager EchoManager
}

type requestTuple struct {
	requestID uint64
	request   Request
}

func (ap *applier) start() {
	go func() {
		for {
			select {
			case tup := <-ap.requestCh:
				reqID := tup.requestID
				req := tup.request
				switch {
				case req.echoRequest != nil:
					rs, err := ap.echoManager.apply(req.echoRequest)
					if err != nil {
						rs = fmt.Sprintf("failed to apply %v", err)
					}
					if err = ap.notifier.trigger(reqID, rs); err != nil {
						fmt.Printf("failed to trigger %v", err)
					}
				default:
				}
			case <-ap.stopCh:
				ap.doneCh <- struct{}{}
				return
			}
		}
	}()
}

func (ap *applier) stop() error {
	select {
	case ap.stopCh <- struct{}{}:
	case <-time.After(5 * time.Second):
		return errors.New("took too long to signal stop")
	}
	select {
	case <-ap.doneCh:
	case <-time.After(5 * time.Second):
		return errors.New("took too long to receive done")
	}
	return nil
}

func (ap *applier) apply(req Request) (string, error) {
	reqID := ap.requestIDGenerator.next()
	respRx, err := ap.notifier.register(reqID)
	if err != nil {
		return "", err
	}

	select {
	case ap.requestCh <- requestTuple{requestID: reqID, request: req}:
	case <-time.After(ap.requestTimeout):
		if err = ap.notifier.trigger(reqID, fmt.Sprintf("failed to schedule %d in time", reqID)); err != nil {
			return "", err
		}
	}

	msg := ""
	select {
	case msg = <-respRx:
	case <-time.After(ap.requestTimeout):
		return "", errors.New("apply timeout")
	}

	return msg, nil
}
```

Not fully grokking Rust ownership model in async, Alan implements the following code, but faced with a bunch of compiler error messages:

```rust
use async_std::task;

pub struct Applier {
    notifier: notify::Notifier,
    ...
}

impl Applier {
    pub fn new(req_timeout: Duration) -> Self {
        ...
        Self {
            ...
            notifier: notify::Notifier::new(),
            ...
        }
    }
    ...

    pub async fn start(&self) -> io::Result<()> {
        task::spawn(apply_async(
            self.notifier,
            ...
        ));
        ...
        Ok(())
    }
    ...


pub async fn apply_async(
    notifier: notify::Notifier,
    ...
) -> io::Result<()> {
  ...
```

```
error[E0507]: cannot move out of `self.notifier` which is behind a shared reference
  --> src/apply.rs:72:13
   |
72 |             self.notifier,
   |             ^^^^^^^^^^^^^ move occurs because `self.notifier` has type `Notifier`, which does not implement the `Copy` trait
```

After discussing with [Barbara][], Alan adds `Arc` to provide a shared ownership between async tasks:

```rust
use async_std::{sync::Arc, task};

pub struct Applier {
    notifier: Arc<notify::Notifier>,
    ...
}

impl Applier {
    pub fn new(req_timeout: Duration) -> Self {
        ...
        Self {
            ...
            notifier: Arc::new(notify::Notifier::new()),
            ...
        }
    }
    ...

    pub async fn start(&self) -> io::Result<()> {
        task::spawn(apply_async(
            self.notifier.clone(),
            ...
        ));
        ...
        Ok(())
    }
    ...


pub async fn apply_async(
    notifier: Arc<notify::Notifier>,
    ...
) -> io::Result<()> {
  ...
```

Alan is satisfied with the compilation success for the moment, but doesn't feel confident about the production readiness of Rust async.

### Implementing HTTP server handler

Familiar with Go standard libraries, Alan implemented the following request handler without any third-party dependencies:

```go
import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
)

type Handler interface {
	start()
}

type handler struct {
	listenerPort uint64
	applier      Applier
}

func (hd *handler) start() {
	hd.applier.start()

	serverMux := http.NewServeMux()
	serverMux.HandleFunc("/echo", hd.wrapFunc(handleRequest))

	httpServer := &http.Server{
		Addr:    fmt.Sprintf(":%d", hd.listenerPort),
		Handler: serverMux,
	}

	tch := make(chan os.Signal, 1)
	signal.Notify(tch, syscall.SIGINT)
	done := make(chan struct{})
	go func() {
		httpServer.Close()
		close(done)
	}()

	if err := httpServer.ListenAndServe(); err != nil {
		fmt.Printf("http server error: %v\n", err)
	}
	select {
	case <-done:
	default:
	}

	if err := hd.applier.stop(); err != nil {
		panic(err)
	}
}

func (hd *handler) wrapFunc(fn func(applier Applier, w http.ResponseWriter, req *http.Request)) func(w http.ResponseWriter, req *http.Request) {
	return func(w http.ResponseWriter, req *http.Request) {
		fn(hd.applier, w, req)
	}
}

func handleRequest(applier Applier, w http.ResponseWriter, req *http.Request) {
	switch req.Method {
	case "POST":
		var echoRequest EchoRequest
		err := json.NewDecoder(req.Body).Decode(&echoRequest)
		if err != nil {
			fmt.Fprintf(w, "failed to read request %v", err)
			return
		}
		s, err := applier.apply(Request{echoRequest: &echoRequest})
		if err != nil {
			fmt.Fprintf(w, "failed to apply request %v", err)
			return
		}
		fmt.Fprint(w, s)

	default:
		http.Error(w, "Method Not Allowed", 405)
	}
}
```

For Rust, Alan has multiple options to build a web server: [hyper](https://github.com/hyperium/hyper), [actix-web](https://github.com/actix/actix-web), [warp](https://github.com/seanmonstar/warp), [rocket](https://github.com/SergioBenitez/Rocket), [tide](https://github.com/http-rs/tide), etc..

Alan strongly believes in Go's minimal dependency approach, and thereby chooses "hyper" for its low-level API. While "hyper" is meant to be a low-level building block, implementing a simple request handler in "hyper" still requires four different external dependencies. Alan is not surprised anymore, and rather accepts the status quo of split Rust ecosystem:

```bash
cargo add http
cargo add futures
cargo add hyper --features full
cargo add tokio --features full
```

After multiple days, Alan finally writes the following code:

```rust
use async_std::sync::Arc;
use futures::TryStreamExt;
use http::{Method, Request, Response, StatusCode, Version};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Server};
use tokio::signal;

pub struct Handler {
    listener_port: u16,
    applier: Arc<apply::Applier>,
}

impl Handler {
    ...
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("starting server");
        match self.applier.start().await {
            Ok(_) => println!("started applier"),
            Err(e) => panic!("failed to stop applier {}", e),
        }

        let addr = ([0, 0, 0, 0], self.listener_port).into();
        let svc = make_service_fn(|socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            let applier = self.applier.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    handle_request(remote_addr, req, applier.clone())
                }))
            }
        });

        let server = Server::bind(&addr)
            .serve(svc)
            .with_graceful_shutdown(handle_sigint());

        if let Err(e) = server.await {
            println!("server error: {}", e);
        }

        match self.applier.stop().await {
            Ok(_) => println!("stopped applier"),
            Err(e) => println!("failed to stop applier {}", e),
        }

        Ok(())
    }
}

async fn handle_request(
    addr: SocketAddr,
    req: Request<Body>,
    applier: Arc<apply::Applier>,
) -> Result<Response<Body>, hyper::Error> {
    let http_version = req.version();
    let method = req.method().clone();
    let cloned_uri = req.uri().clone();
    let path = cloned_uri.path();

    let resp = match http_version {
        Version::HTTP_11 => {
            match method {
                Method::POST => {
                    let mut resp = Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)...
                    match req
                        .into_body()
                        .try_fold(Vec::new(), |mut data, chunk| async move {
                            data.extend_from_slice(&chunk);
                            Ok(data)
                        })
                        .await
                    {
                        Ok(body) => {
                            let mut success = false;
                            let mut req = apply::Request::new();
                            match path {
                                "/echo" => match echo::parse_request(&body) {
                                    Ok(bb) => {
                                        req.echo_request = Some(bb);
                                        success = true;
                                    }
                                    Err(e) => {
                                        resp = Response::builder()
                                            .status(StatusCode::INTERNAL_SERVER_ERROR)...
                                    }
                                },
                                _ => {
                                    println!("unknown path {}", path);
                                    resp = Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)...
                                }
                            }
                            if success {
                                match applier.apply(req).await {
                                    Ok(rs) => resp = Response::new(Body::from(rs)),
                                    Err(e) => {
                                        resp = Response::builder()
                                            .status(StatusCode::INTERNAL_SERVER_ERROR)...
                                    }
                                }
                            }
                        }
                        Err(e) => ...
                    }
                    resp
                }

                _ => Response::builder()
                    .status(StatusCode::NOT_FOUND)...
            }
        }

        _ => Response::builder()
            .status(StatusCode::HTTP_VERSION_NOT_SUPPORTED)...
    };
    Ok(resp)
}
```

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

Alan's trust in Go mainly comes from its consistent and coherent approach to the problems. Alan prefers a standard way of doing things and as a result, multiple libraries available for async Rust caused Alan confusion. For instance, [etcd](https://github.com/etcd-io/etcd) relies on Go's standard HTTP libraries for HTTP/1 and [grpc-go](https://github.com/grpc/grpc-go) for HTTP/2 which is used by many other Go projects. The core networking library [`golang.org/x/net`](https://github.com/golang/net) is actively maintained by Go team with common interests from the community.

The existing Rust syntax becomes more unwieldy and complicated to use for async Rust code. To make things worse, the lack of coherence in Rust async ecosystem can easily undermine basic user trust in a significant way. 

### **What are the sources for this story?**

- Years of experience building a distributed key-value store in Go, [etcd](https://github.com/etcd-io/etcd).
- Simplified etcd server implementation in Go and Rust can be found at [gyuho/task-scheduler-examples](https://github.com/gyuho/task-scheduler-examples).

### **Why did you choose Alan to tell this story?**

I chose Alan because he is used to Go, where these issues play out differently. Go natively supports: (1) asynchronous task with "goroutine", (2) asynchronous communication with "channel", and (3) performant HTTP server library. Each component is nicely composed together. There is no need to worry about picking the right external dependencies or resolving dependency conflicts. Concurrency being treated as first-class by Go maintainers built great confidence in Alan's decision to use Go.

### **How would this story have played out differently for the other characters?**

This story would likely have played out the same for almost everyone new to Rust (except [Barbara][]).

[distridata]: ../projects/DistriData.md
[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
