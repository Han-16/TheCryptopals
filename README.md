# the cryptopals crypto challenges
<h1>Welcome to the challenges</h1>

We can't introduce these any better than Maciej Ceglowski did, so read that blog post first.<br>

We've built a collection of exercises that demonstrate attacks on real-world crypto.<br>

This is a different way to learn about crypto than taking a class or reading a book. We give you problems to solve. They're derived from weaknesses in real-world systems and modern cryptographic constructions. We give you enough info to learn about the underlying crypto concepts yourself. When you're finished, you'll not only have learned a good deal about how cryptosystems are built, but you'll also understand how they're attacked.

<h2>What Are The Rules?</h2>
There aren't any! For several years, we ran these challenges over email, and asked participants not to share their results. The honor system worked beautifully! But now we're ready to set aside the ceremony and just publish the challenges for everyone to work on.

<h2>How Much Math Do I Need To Know?</h2>
If you have any trouble with the math in these problems, you should be able to find a local 9th grader to help you out. It turns out that many modern crypto attacks don't involve much hard math.


<h2>So What Do I Need To Know?</h2>
You'll want to be able to code proficiently in any language. We've received submissions in C, C++, Python, Ruby, Perl, Visual Basic, X86 Assembly, Haskell, and Lisp. Surprise us with another language. Our friend Maciej says these challenges are a good way to learn a new language, so maybe now's the time to pick up Clojure or Rust.

<h2>What Should I Expect?</h2>
Right now, we have eight sets. They get progressively harder. Again: these are based off real-world vulnerabilities. None of them are "puzzles". They're not designed to trip you up. Some of the attacks are clever, though, and if you're not familiar with crypto cleverness... well, you should like solving puzzles. An appreciation for early-90's MTV hip-hop can't hurt either.

<h2>Can You Give Us A Long-Winded Indulgent Description For Why You'Ve Chosen To Do This?</h2>
It turns out that we can.<br><br>

If you're not that familiar with crypto already, or if your familiarity comes mostly from things like Applied Cryptography, this fact may surprise you: most crypto is fatally broken. The systems we're relying on today that aren't known to be fatally broken are in a state of just waiting to be fatally broken. Nobody is sure that TLS 1.2 or SSH 2 or OTR are going to remain safe as designed.<br>

The current state of crypto software security is similar to the state of software security in the 1990s. Specifically: until around 1995, it was not common knowledge that software built by humans might have trouble counting. As a result, nobody could size a buffer properly, and humanity incurred billions of dollars in cleanup after a decade and a half of emergency fixes for memory corruption vulnerabilities.<br>

Counting is not a hard problem. But cryptography is. There are just a few things you can screw up to get the size of a buffer wrong. There are tens, probably hundreds, of obscure little things you can do to take a cryptosystem that should be secure even against an adversary with more CPU cores than there are atoms in the solar system, and make it solveable with a Perl script and 15 seconds. Don't take our word for it: do the challenges and you'll see.<br>

People "know" this already, but they don't really know it in their gut, and we think the reason for that is that very few people actually know how to implement the best-known attacks. So, mail us, and we'll give you a tour of them.

