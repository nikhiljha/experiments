kubeajsd:
  .data
msg:
  .asciz "Hello, world!\n"
meme:
  .text
  .global _main
_main: mov $0x02000004, %rax
kubeaa: mov $1, %rdi /*
apiVersion: apps/v1
kind: Deployment
metadata:
  kube2: aa */
  name: nop
spec:
  kube5: mov msg@GOTPCREL(%rip), %rsi /*
  replicas: 1
  selector:
    matchLabels:
      app: Nop
  kube6: aa */
  template:
    metadata:
      labels:
        app: Nop
    kube3: mov $13, %rdx /*
    spec:
      containers:
      - image: gcr.io/google_containers/echoserver:1.0
        imagePullPolicy: Always
        name: echoserver
        ports:
        - containerPort: 8080
    kube4: aa */
    kube9:
      syscall
      mov   $0x02000001, %rax
      xor   %rdi, %rdi
      syscall
