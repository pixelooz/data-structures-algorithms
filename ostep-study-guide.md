# OSTEP Study Guide for Fresher Systems and Engineering Interviews

**Book:** [Operating Systems: Three Easy Pieces](https://pages.cs.wisc.edu/~remzi/OSTEP/) (free, version 1.10). Chapter numbers below follow that edition.

**Short answer:** study about 20 chapters closely (Tier 1), about 16 for systems and infra roles (Tier 2), and skip the rest unless a role asks for it (Tier 3). The book has roughly 50 chapters, but only a minority carry the questions freshers actually get.

> **How this list was chosen:** no source ranks OSTEP chapters by interview frequency, so the tiers are my judgment. The topic list is consistent with fresher OS interview guides, which put the same themes up front: process vs thread, scheduling algorithms, deadlocks, memory management, and virtual memory ([Kaashiv Infotech guide](https://www.kaashivinfotech.com/blog/?p=21666), [GoLinuxCloud guide](https://www.golinuxcloud.com/operating-system-interview-questions/)). Expected depth also depends on the role: for a general backend job it may stop at processes vs threads, virtual memory, and deadlock ([PracHub](https://prachub.com/resources/operating-systems-interview-questions-for-software-engineers-processes-memory-scheduling-and-concurrency)).

## How to use this

| Step | What to do |
|---|---|
| 1. Read | Read the chapter once. Skip the long asides on first pass. |
| 2. Run | Do the matching homework in [ostep-homework](https://github.com/remzi-arpacidusseau/ostep-homework) (most are simulators you run from the command line). Details are on the [homework page](https://pages.cs.wisc.edu/~remzi/OSTEP/Homework/homework.html). |
| 3. Explain | Answer the "Be able to explain" column out loud without notes. If you can't, reread. |
| 4. Drill | After each tier, run through an interview question list: [GoLinuxCloud](https://www.golinuxcloud.com/operating-system-interview-questions/), [PracHub](https://prachub.com/resources/operating-systems-interview-questions-for-software-engineers-processes-memory-scheduling-and-concurrency), [LeetCode discuss list](https://leetcode.com/discuss/interview-question/operating-system/3583103/Topic-Wise-commonly-asked-operating-system-interview-questionsoror-Topic-Wise-Arranged). |

For hands-on depth beyond the simulators, the same authors maintain [ostep-projects](https://github.com/remzi-arpacidusseau/ostep-projects), and MIT's xv6-based course ([6.S081](https://pdos.csail.mit.edu/6.S081/)) is the classic next step.

**Tailoring note:** if you've already read *Rust Atomics and Locks*, chapters 28-31 will go fast. Focus there on the OS framing (condition variables, semaphores, deadlock conditions), which interviewers ask about and that book frames differently.

**Legend:** ⬜ = tick (✅) when you can explain it cold. Links in the Chapter column are the book's own PDFs. 🎥 = related video by Arpit Bhayani. 📗 = covered in more depth by DDIA.

---

## Tier 1: Must-know (asked in most OS interviews)

### Virtualization: CPU

| ⬜ | Ch. | Chapter | Be able to explain | Hands-on / related |
|:---:|:---:|---|---|---|
| ⬜ | 2 | [Introduction to Operating Systems](https://pages.cs.wisc.edu/~remzi/OSTEP/intro.pdf) | What an OS does (virtualizes and manages resources), system calls, protection | Skim only |
| ⬜ | 4 | [The Abstraction: The Process](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-intro.pdf) | Process vs program, process states, what's in a PCB | `process-run.py` |
| ⬜ | 5 | [Interlude: Process API](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-api.pdf) | `fork`/`exec`/`wait`, why fork and exec are separate (shell redirection, pipes), zombie and orphan processes | `fork.py` + write some code |
| ⬜ | 6 | [Mechanism: Limited Direct Execution](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-mechanisms.pdf) | User vs kernel mode, traps and system calls, timer interrupt and preemption, exactly what a context switch saves and restores | Write some code (measure syscall and switch cost) |
| ⬜ | 7 | [Scheduling: Introduction](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched.pdf) | Turnaround vs response time, FIFO, SJF, STCF, Round Robin, convoy effect | `scheduler.py` |
| ⬜ | 8 | [Scheduling: The Multi-Level Feedback Queue](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-mlfq.pdf) | The MLFQ rules, starvation, gaming the scheduler, priority boost | `mlfq.py` |

### Virtualization: Memory

| ⬜ | Ch. | Chapter | Be able to explain | Hands-on / related |
|:---:|:---:|---|---|---|
| ⬜ | 13 | [The Abstraction: Address Spaces](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-intro.pdf) | Code/heap/stack layout, virtual vs physical addresses, goals: transparency, efficiency, protection | Write some code |
| ⬜ | 14 | [Interlude: Memory API](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-api.pdf) | Stack vs heap, `malloc`/`free`, common bugs (leak, dangling pointer, double free, overflow) | Write some code (run under valgrind or ASan) |
| ⬜ | 15 | [Mechanism: Address Translation](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-mechanism.pdf) | Base and bounds, what the hardware does vs what the OS does | `relocation.py` |
| ⬜ | 18 | [Paging: Introduction](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-paging.pdf) | Pages and frames, page table, PTE bits (valid, present, dirty, protection), why paging is slow without help, page table size | `paging-linear-translate.py` |
| ⬜ | 19 | [Paging: Faster Translations (TLBs)](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-tlbs.pdf) | TLB hit and miss, why it works (locality), what happens on a context switch (flush or ASIDs) | Write some code (measure TLB effects) |
| ⬜ | 20 | [Paging: Smaller Tables](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-smalltables.pdf) | Multi-level page tables: why they save space and what they cost | `paging-multilevel-translate.py` |
| ⬜ | 21 | [Beyond Physical Memory: Mechanisms](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-beyondphys.pdf) | Swap space, the present bit, the full page-fault path, what the OS does on a fault | `mem.c` |
| ⬜ | 22 | [Beyond Physical Memory: Policies](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-beyondphys-policy.pdf) | Optimal, FIFO, Random, LRU, Clock, Belady's anomaly, thrashing | `paging-policy.py` |

### Concurrency

| ⬜ | Ch. | Chapter | Be able to explain | Hands-on / related |
|:---:|:---:|---|---|---|
| ⬜ | 26 | [Concurrency: An Introduction](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-intro.pdf) | Thread vs process, shared address space, race conditions, critical sections, atomicity | ostep-homework `threads-intro`; 🎥 [Why `count++` isn't atomic](https://arpitbhayani.me/videos/why-count-plus-plus-is-not-atomic-assembly-micro-operations) |
| ⬜ | 27 | [Interlude: Thread API](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-api.pdf) | `pthread_create`/`join`, how to use locks and condition variables correctly | ostep-homework `threads-api`; 🎥 [Multi-threaded TCP server](https://arpitbhayani.me/videos/building-multithreaded-tcp-server-from-scratch) |
| ⬜ | 28 | [Locks](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-locks.pdf) | Test-and-set, compare-and-swap, fetch-and-add, spin locks vs sleeping locks, futex, how to evaluate a lock | ostep-homework `threads-locks`; 🎥 [Mutexes and pessimistic locking](https://arpitbhayani.me/videos/understanding-pessimistic-locking-with-mutex), [CAS](https://arpitbhayani.me/videos/optimistic-locking-internals-cas) |
| ⬜ | 30 | [Condition Variables](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-cv.pdf) | `wait`/`signal`, producer-consumer, why you wait in a `while` loop (Mesa semantics), spurious wakeups | ostep-homework `threads-cv`; 🎥 [Thread-safe queue from scratch](https://arpitbhayani.me/videos/concurrent-thread-safe-queue-from-scratch) |
| ⬜ | 31 | [Semaphores](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-sema.pdf) | Semaphore as lock and as ordering tool, producer-consumer, reader-writer, dining philosophers, building one from a lock plus a CV | ostep-homework `threads-sema` |
| ⬜ | 32 | [Common Concurrency Problems](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-bugs.pdf) | Atomicity and order violations, the four deadlock conditions, prevention (lock ordering), avoidance, detection | Run some C code (ostep-homework `threads-bugs`); 🎥 [Writing deadlock-free code](https://arpitbhayani.me/videos/writing-deadlock-free-code) |

---

## Tier 2: Likely for systems and infra roles

### Virtualization (deeper)

| ⬜ | Ch. | Chapter | Be able to explain | Hands-on / related |
|:---:|:---:|---|---|---|
| ⬜ | 9 | [Scheduling: Proportional Share](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-lottery.pdf) | Lottery and stride scheduling, the idea behind fair-share schedulers like Linux's | `lottery.py` |
| ⬜ | 10 | [Multiprocessor Scheduling](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-multi.pdf) | Cache affinity, single-queue vs multi-queue, load balancing | `multi.py` |
| ⬜ | 16 | [Segmentation](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-segmentation.pdf) | Segmentation vs paging, external fragmentation (the "paging vs segmentation" question) | `segmentation.py` |
| ⬜ | 17 | [Free-Space Management](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-freespace.pdf) | Allocator strategies (first/best/worst fit), splitting and coalescing, how `malloc` is built | `malloc.py` |
| ⬜ | 23 | [Complete VM Systems](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-complete.pdf) | Real-world VM: demand zeroing, copy-on-write (why `fork` is cheap), the Linux address space | Read only |

### Concurrency (deeper)

| ⬜ | Ch. | Chapter | Be able to explain | Hands-on / related |
|:---:|:---:|---|---|---|
| ⬜ | 29 | [Lock-based Concurrent Data Structures](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-locks-usage.pdf) | Concurrent counters, queues, hash tables, why one big lock doesn't scale | ostep-homework `threads-locks-usage`; 🎥 [Thread pools](https://arpitbhayani.me/videos/why-thread-pools-exist-and-how-to-implement-them) |
| ⬜ | 33 | [Event-based Concurrency](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-events.pdf) | Event loops, `select`/`poll`, blocking calls in event code, async I/O | Write some code; 🎥 [Async I/O with epoll](https://arpitbhayani.me/videos/implementing-asynchronous-io-epoll-concurrent-redis-server), [Event loop internals](https://arpitbhayani.me/videos/event-loops-internals-redis-concurrent-connections) |

### Persistence

| ⬜ | Ch. | Chapter | Be able to explain | Hands-on / related |
|:---:|:---:|---|---|---|
| ⬜ | 36 | [I/O Devices](https://pages.cs.wisc.edu/~remzi/OSTEP/file-devices.pdf) | Polling vs interrupts, DMA, device drivers, how the OS talks to hardware | Read only |
| ⬜ | 37 | [Hard Disk Drives](https://pages.cs.wisc.edu/~remzi/OSTEP/file-disks.pdf) | Seek, rotation, transfer cost, why sequential beats random, disk scheduling | `disk.py` |
| ⬜ | 38 | [Redundant Disk Arrays (RAID)](https://pages.cs.wisc.edu/~remzi/OSTEP/file-raid.pdf) | RAID 0/1/4/5 trade-offs (capacity, reliability, performance) | `raid.py` |
| ⬜ | 39 | [Interlude: Files and Directories](https://pages.cs.wisc.edu/~remzi/OSTEP/file-intro.pdf) | File descriptors, `open`/`read`/`write`/`fsync`, hard vs soft links, mounting | Write some code |
| ⬜ | 40 | [File System Implementation](https://pages.cs.wisc.edu/~remzi/OSTEP/file-implementation.pdf) | Inodes, directories, free-space tracking, what `open` and `read` actually do on disk, caching | `vsfs.py` |
| ⬜ | 41 | [Locality and the Fast File System](https://pages.cs.wisc.edu/~remzi/OSTEP/file-ffs.pdf) | Cylinder/block groups, why placement matters (skim) | `ffs.py` |
| ⬜ | 42 | [Crash Consistency: FSCK and Journaling](https://pages.cs.wisc.edu/~remzi/OSTEP/file-journaling.pdf) | The crash-consistency problem, fsck, write-ahead logging, data vs metadata journaling (the same idea databases use) | `fsck.py`; 🎥 [Write-ahead logging](https://arpitbhayani.me/videos/demystifying-write-ahead-logging-database-reliability); 📗 DDIA: Storage and Retrieval |
| ⬜ | 43 | [Log-structured File Systems](https://pages.cs.wisc.edu/~remzi/OSTEP/file-lfs.pdf) | Sequential writes, segments, garbage collection (the ancestor of LSM trees) | `lfs.py`; 📗 DDIA: Storage and Retrieval |
| ⬜ | 44 | [Flash-based SSDs](https://pages.cs.wisc.edu/~remzi/OSTEP/file-ssd.pdf) | Erase before write, the flash translation layer, write amplification, wear leveling | `ssd.py` |
| ⬜ | 45 | [Data Integrity and Protection](https://pages.cs.wisc.edu/~remzi/OSTEP/file-integrity.pdf) | Checksums, silent corruption, scrubbing | Read only |

---

## Tier 3: Skip for now (read only if a role asks)

| Chapters | Why skip |
|---|---|
| 1, 3, 12, 25, 35, 47 (the "Dialogue" chapters) | Short conceptual openers. Read one for fun, but they carry no interview content. |
| 11, 24, 34, 46, 51 (the "Summary Dialogue" chapters) | Recaps of material you'll already have covered. |
| 48-50 (Distributed Systems, NFS, AFS) | Covered better and more usefully by your system design guide, and distributed topics are rarely OS-fresher questions. |
| Security part (chapters added by Peter Reiher, UCLA) | Skip unless the role is security-adjacent. |

---

## What OSTEP doesn't cover (but interviews sometimes do)

| Topic | Where to look |
|---|---|
| Inter-process communication (pipes, shared memory, message queues, sockets) | [Beej's Guide to Unix IPC](https://beej.us/guide/bgipc/) |
| Linux specifics: containers, cgroups, namespaces, page cache, system calls | [man7.org man pages](https://man7.org/linux/man-pages/) and *The Linux Programming Interface* (book) |
| Hands-on kernel work | [6.S081 (xv6)](https://pdos.csail.mit.edu/6.S081/) |

---

## Suggested order

1. Tier 1 **CPU virtualization** → **memory virtualization** → **concurrency**, running each chapter's homework before moving on.
2. Drill interview questions on Tier 1 (step 4 above) before starting Tier 2.
3. Tier 2 **persistence** next (chapters 42-43 map directly onto database and storage-engine work), then the deeper virtualization and concurrency chapters.
4. Leave Tier 3 alone.
