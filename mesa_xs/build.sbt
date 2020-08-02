
// The simplest possible sbt build file is just one line:

scalaVersion := "2.13.1"
// That is, to create a valid sbt build, all you've got to do is define the
// version of Scala you'd like your project to use.

// ============================================================================

// Lines like the above defining `scalaVersion` are called "settings". Settings
// are key/value pairs. In the case of `scalaVersion`, the key is "scalaVersion"
// and the value is "2.13.1"

// It's possible to define many kinds of settings, such as:

name := "mesax"
organization := "org.mesa-x"
version := "0.1"

// Want to use a published library in your project?
// You can define other libraries as dependencies in your build like this:
// libraryDependencies += "org.typelevel" %% "cats-core" % "2.0.0"
// Here, `libraryDependencies` is a set of dependencies, and by using `+=`,
// we're adding the cats dependency to the set of dependencies that sbt will go
// and fetch when it starts up.
// Now, in any Scala file, you can import classes, objects, etc., from cats with
// a regular import.

// TIP: To find the "dependency" that you need to add to the
// `libraryDependencies` set, which in the above example looks like this:


libraryDependencies ++= Seq("org.specs2" %% "specs2-core" % "4.10.0" % "test")

// https://mvnrepository.com/artifact/net.liftweb/lift-webkit
libraryDependencies += "net.liftweb" %% "lift-util" % "3.4.1"
libraryDependencies += "com.lihaoyi" %% "fastparse" % "2.3.0"

scalacOptions in Test ++= Seq("-Yrangepos")


// IMPORTANT NOTE: while build files look _kind of_ like regular Scala, it's
// important to note that syntax in *.sbt files doesn't always behave like
// regular Scala. For example, notice in this build file that it's not required
// to put our settings into an enclosing object or class. Always remember that
// sbt is a bit different, semantically, than vanilla Scala.

// ============================================================================

