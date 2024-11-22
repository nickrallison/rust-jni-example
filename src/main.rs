
use jni::JNIEnv;
use jni::objects::JClass;
use jni::JavaVM;
use jni::InitArgsBuilder;

fn main() {

    // Set up the JVM arguments, including the classpath to your compiled Java classes
    let jvm_args = InitArgsBuilder::new()
        .version(jni::JNIVersion::V8) // Use Java 8 (adjust if you have a different Java version)
        .option("-Djava.class.path=target/classes") // Set classpath
        .build()
        .unwrap();

    // Create a new JVM
    let jvm = JavaVM::new(jvm_args).unwrap();

    // Attach the current thread to the JVM to obtain a `JNIEnv` instance
    let mut env = jvm.attach_current_thread().unwrap();

    // Find the Java class `com.me.Test`
    let class = env.find_class("com/me/Test").unwrap();

    // Call the static method `hello` with the signature `()V` (no arguments, void return type)
    env.call_static_method(class, "hello", "()V", &[]).unwrap();

}