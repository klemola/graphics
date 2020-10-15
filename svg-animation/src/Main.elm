module Main exposing (main)

import Animator
import Browser
import Browser.Events exposing (onAnimationFrame)
import Collage exposing (circle, filled, rectangle, rotate, scale, shift, uniform)
import Collage.Layout exposing (stack)
import Collage.Render exposing (svg)
import Color
import Html exposing (Html)
import Html.Events exposing (onClick)
import Time


type Direction
    = Up
    | Right
    | Down
    | Left


type Status
    = Turning { from : Direction, to : Direction }
    | Moving


type alias Position =
    ( Float, Float )


type alias Thing =
    { position : Position
    , status : Status
    , rotation : Float
    }


type alias Model =
    { currentScale : Animator.Timeline Float
    , thing : Thing
    , thingBeforeTurn : Maybe Thing
    }


init =
    let
        thing =
            { position = ( 50, 0 )
            , status = Turning { from = Right, to = Up }
            , rotation = degrees (dirToDegrees Right)
            }
    in
    ( { currentScale = Animator.init 0
      , thing = thing
      , thingBeforeTurn = Just thing
      }
    , Cmd.none
    )


main =
    Browser.document
        { init = \() -> init
        , view = view
        , update = update
        , subscriptions =
            \model ->
                onAnimationFrame Tick
        }


type Msg
    = Tick Time.Posix
    | Scale
    | Turn Direction
    | Reset


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Tick newTime ->
            ( { model
                | currentScale = Animator.updateTimeline newTime model.currentScale
                , thing = nextThing model.thing model.thingBeforeTurn
              }
            , Cmd.none
            )

        Scale ->
            ( { model
                | currentScale =
                    model.currentScale
                        |> Animator.go (Animator.millis 750) 1
              }
            , Cmd.none
            )

        Turn target ->
            let
                thing =
                    model.thing

                updatedThing =
                    { thing | status = Turning { from = Debug.log "dir" (radiansToClosestDir thing.rotation), to = target } }
            in
            ( { model | thing = updatedThing }, Cmd.none )

        Reset ->
            init


nextThing : Thing -> Maybe Thing -> Thing
nextThing thing thingBeforeTurn =
    let
        { position, status, rotation } =
            thing
    in
    case status of
        Moving ->
            if (Tuple.first position |> abs) >= 100 || (Tuple.second position |> abs) >= 100 then
                thing

            else
                Thing (rotationToMovement rotation position) Moving rotation

        Turning { from, to } ->
            if turningComplete to rotation then
                Thing position Moving rotation

            else
                let
                    ( nextPosition, nextRotation ) =
                        case thingBeforeTurn of
                            Just beforeTurn ->
                                turn
                                    { from = from
                                    , to = to
                                    , rotation = rotation
                                    , position = position
                                    , originalPosition = beforeTurn.position
                                    }

                            Nothing ->
                                ( position, rotation )
                in
                { thing | position = nextPosition, rotation = nextRotation }


rotationToMovement : Float -> Position -> Position
rotationToMovement rotation ( x, y ) =
    let
        rotationInPolar =
            rotation + degrees 90

        rotationWithLimit =
            -- adjust rotation for pre-polar adjusted value of < 270°
            if rotationInPolar - degrees 360 >= 0 then
                rotationInPolar - degrees 360

            else
                rotationInPolar

        ( addX, addY ) =
            fromPolar ( 1, rotationWithLimit )
    in
    ( x + addX, y + addY )


turningComplete : Direction -> Float -> Bool
turningComplete dir rotation =
    let
        current =
            if rotation - degrees 360 >= 0 then
                0

            else
                rotation

        target =
            degrees (dirToDegrees dir)
    in
    target == current


turnAreaSize =
    50.0


rotationPerStep =
    degrees (90 / turnAreaSize)


turn :
    { from : Direction
    , to : Direction
    , rotation : Float
    , position : Position
    , originalPosition : Position
    }
    -> ( Position, Float )
turn { from, to, rotation, position, originalPosition } =
    let
        ( x, y ) =
            position

        ( originalX, originalY ) =
            originalPosition

        target =
            degrees (dirToDegrees to)

        rotated =
            if target == 0 then
                min (degrees 360) (rotation + rotationPerStep)

            else
                min target (rotation + rotationPerStep)

        -- foo =
        --     Debug.log "target, from/to" ( target, from, to )
        -- bar =
        --     Debug.log "rotation / ted" ( rotation, rotated )
        nextPosition =
            case ( from, to ) of
                ( Right, Up ) ->
                    ( x + 1, smoothCurve x (originalY + turnAreaSize) )

                ( Up, Left ) ->
                    ( smoothCurve2 x turnAreaSize, y + 1 )

                _ ->
                    position
    in
    ( nextPosition
    , if rotated >= degrees 360 then
        0

      else
        rotated
    )


smoothCurve : Float -> Float -> Float
smoothCurve distance area =
    ((distance / area) ^ 2) * area


smoothCurve2 : Float -> Float -> Float
smoothCurve2 distance area =
    ((distance / area) ^ 2) * area


dirToDegrees : Direction -> Float
dirToDegrees dir =
    case dir of
        Up ->
            0

        Right ->
            270

        Down ->
            180

        Left ->
            90


radiansToClosestDir : Float -> Direction
radiansToClosestDir rad =
    if rad == 0 then
        Up

    else
        let
            piOver4 =
                pi / 4

            cardinalRad =
                piOver4 * (toFloat <| round (rad / piOver4))
        in
        case round cardinalRad of
            270 ->
                Right

            180 ->
                Down

            90 ->
                Left

            _ ->
                Up


view : Model -> Browser.Document Msg
view model =
    let
        thing =
            model.thing

        currentScale =
            Animator.linear model.currentScale
                (\state ->
                    Animator.at state
                        |> Animator.leaveSmoothly 0.3
                        |> Animator.withWobble 1
                )

        container =
            rectangle 500 500
                |> filled (uniform Color.red)

        rect =
            rectangle 100 100
                |> filled (uniform Color.blue)
                |> scale currentScale

        renderedThing =
            stack
                [ circle 5
                    |> filled (uniform Color.white)
                    |> shift ( 0, 15.0 )
                , circle 20
                    |> filled (uniform Color.black)
                ]
                |> rotate thing.rotation
                |> shift thing.position

        status =
            case thing.status of
                Moving ->
                    "Moving"

                Turning _ ->
                    "Turning"
    in
    { title = "Scale"
    , body =
        [ currentScale
            |> String.fromFloat
            |> Html.text
        , Html.div []
            [ thing.rotation
                |> String.fromFloat
                |> Html.text
            ]
        , Html.div []
            [ Html.text status
            ]
        , Html.div []
            [ Html.text (Debug.toString thing.position)
            ]
        , Html.button [ onClick Scale ] [ Html.text "Click me" ]
        , Html.button [ onClick Reset ] [ Html.text "Reset" ]
        , Html.button [ onClick (Turn Left) ] [ Html.text "Turn left" ]
        , stack [ renderedThing, rect, container ] |> svg
        ]
    }
