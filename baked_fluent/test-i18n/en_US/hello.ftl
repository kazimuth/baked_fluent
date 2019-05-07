# english translations
greeting = Hello { $name }! { $friends ->
    [one] You have a friend!
    [zero] You have no friends yet 😞
   *[other] You have {$friends} friends.
}
