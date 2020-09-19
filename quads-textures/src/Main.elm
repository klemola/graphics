module Main exposing (main)

import Array
import Browser
import Camera3d
import Color exposing (Color)
import Direction3d
import Html
import Length
import Pixels
import Point3d
import Scene3d
import Scene3d.Material as Material
import Scene3d.Mesh as Mesh
import Task
import TriangularMesh
import Viewpoint3d
import WebGL.Texture


tileSheetSizePx : Float
tileSheetSizePx =
    512.0


tileSizePx : Float
tileSizePx =
    128.0


exampleMesh =
    tileMeshByIndex 13



{- Selects a tile based on 0-index location in a tilesheet.
   Index 0 is a tile in the top left corner of the tilesheet.
-}


tileMeshByIndex : Int -> Mesh.Unlit coordinates
tileMeshByIndex idx =
    let
        col =
            if idx < 4 then
                idx + 1

            else
                modBy 4 (idx + 1)

        row =
            (idx // 4)
                |> toFloat
                |> floor
                |> (+) 1
    in
    ( col, row )
        |> tilePositionToUVs
        |> tileMesh



{- Maps coordinates (columns/rows) in a tile sheet to a square tile of x width/height, given
   a tilesheet of y width/height.
-}


type alias UVs =
    { bottomLeft : ( Float, Float )
    , bottomRight : ( Float, Float )
    , topLeft : ( Float, Float )
    , topRight : ( Float, Float )
    }


tilePositionToUVs : ( Int, Int ) -> UVs
tilePositionToUVs ( col, row ) =
    let
        ( colInScale, rowInScale ) =
            ( toFloat col * tileSizePx, toFloat row * tileSizePx )

        -- cols are positive numbers, while position starts from 0
        -- shift col "one to the left"
        originX =
            colInScale - tileSizePx

        -- UVs work y values that increase from bottom to top, while rows work in the opposite direction
        -- shift row by the size of the tilesheet
        originY =
            abs (rowInScale - tileSheetSizePx)
    in
    { bottomLeft = ( originX / tileSheetSizePx, originY / tileSheetSizePx )
    , bottomRight = ( originX / tileSheetSizePx, (originY + tileSizePx) / tileSheetSizePx )
    , topLeft = ( (originX + tileSizePx) / tileSheetSizePx, originY / tileSheetSizePx )
    , topRight = ( (originX + tileSizePx) / tileSheetSizePx, (originY + tileSizePx) / tileSheetSizePx )
    }



{- Builds a quad covered with a subtexture, by UV coordinates. -}


tileMesh : UVs -> Mesh.Unlit coordinates
tileMesh uvs =
    let
        vertices =
            Array.fromList
                [ { position = Point3d.meters 0 1 0, uv = uvs.bottomLeft }
                , { position = Point3d.meters 1 1 0, uv = uvs.bottomRight }
                , { position = Point3d.meters 0 0 0, uv = uvs.topLeft }
                , { position = Point3d.meters 1 0 0, uv = uvs.topRight }
                ]

        faces =
            [ ( 0, 1, 3 ), ( 0, 3, 2 ) ]
    in
    TriangularMesh.indexed vertices faces
        |> Mesh.texturedTriangles


type Model
    = Loading -- Waiting for texture to load
    | Loaded (Material.Texture Color) -- Successfully loaded texture
    | Error WebGL.Texture.Error -- Error occurred when loading texture


type Msg
    = GotTexture (Result WebGL.Texture.Error (Material.Texture Color))


textureUrl : String
textureUrl =
    "/assets/road-tilesheet_annotated.png"


init : () -> ( Model, Cmd Msg )
init () =
    -- Attempt to load texture, using trilinear filtering for maximum smoothness
    ( Loading
    , Task.attempt GotTexture (Material.loadWith Material.trilinearFiltering textureUrl)
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        GotTexture (Ok texture) ->
            -- Successfully loaded the texture
            ( Loaded texture, Cmd.none )

        GotTexture (Err error) ->
            -- Network error, bad image dimensions etc.
            ( Error error, Cmd.none )


view : Model -> Browser.Document Msg
view model =
    let
        -- Construct a fixed camera
        camera =
            Camera3d.orthographic
                { viewpoint =
                    Viewpoint3d.lookAt
                        { focalPoint = Point3d.meters 0 0 0
                        , eyePoint = Point3d.meters 0 0 5
                        , upDirection = Direction3d.positiveZ
                        }
                , viewportHeight = Length.meters 10
                }
    in
    { title = "Texture"
    , body =
        case model of
            Loading ->
                -- Waiting for texture to load
                [ Html.text "Loading..." ]

            Loaded texture ->
                let
                    tileSheet =
                        Material.texturedColor texture
                in
                -- Texture loaded successfully, render a scene using it
                [ Scene3d.unlit
                    { camera = camera
                    , dimensions = ( Pixels.int 1024, Pixels.int 1024 )
                    , background = Scene3d.backgroundColor <| Color.rgb255 22 22 22
                    , clipDepth = Length.meters 0.1
                    , entities =
                        [ Scene3d.mesh tileSheet exampleMesh
                        ]
                    }
                ]

            Error error ->
                -- Use a placeholder error message if texture loading fails
                [ Html.text (Debug.toString error) ]
    }


main : Program () Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
